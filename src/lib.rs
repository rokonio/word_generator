// By Rokonio <naelkanada+rust@gmail.com>
// This crate is under the MIT liscence <LISCENCE or
// https://opensource.org/licenses/MIT>

//! Utilities for generating random words based on a language
//!
//! Word_generator provides utilities for generating words based on a language.
//! It create a ProbabilityTable (see
//! [Markov Chains](https://en.wikipedia.org/wiki/Markov_chain#Examples)) by
//! analyzing a language that can then generate words that souds like this
//! language.
//!
//! # Example
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use std::{fs::File, io::BufReader};
//! use word_generator::*;
//!
//! let reader = BufReader::new(File::open("Fr.txt")?);
//! let table = ProbabilityTable::from_reader(reader, 3)?;
//!
//! println!("{:?}", table.generate_words(15)); // Generate 15 word
//! // Is the same as this
//! // println!("{:?}", generate_words(reader, 3, 15));
//! # Ok(())
//! # }
//! ```
use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

/// This type represents represents the probability table of each caracters for
/// a language to appears after a given string of a certain length.
///
/// It is used as a generator to create words that souds like the language that
/// was analyzed.
///
/// # Example
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// use std::{fs::File, io::BufReader};
/// use word_generator::*;
///
/// let reader = BufReader::new(File::open("Fr.txt")?);
/// let table = ProbabilityTable::from_reader(reader, 3)?;
///
/// println!("{:?}", table.generate_words(15)); // Generate 15 word
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ProbabilityTable {
    table: HashMap<String, HashMap<char, u32>>,
    input_length: usize,
}

impl ProbabilityTable {
    fn new(input_length: usize) -> ProbabilityTable {
        ProbabilityTable {
            table: HashMap::new(),
            input_length,
        }
    }

    /// Construct a new `ProbabilityTable` from reader. It can be any type that
    /// implements [`BufRead`].
    pub fn from_reader(reader: impl BufRead, input_length: usize) -> io::Result<ProbabilityTable> {
        Ok(generate_table(
            add_space(reader, input_length)?,
            input_length,
        ))
    }

    /// Generate `amount` words.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// use std::{fs::File, io::BufReader};
    /// use word_generator::*;
    ///
    /// let reader = BufReader::new(File::open("Fr.txt")?);
    /// let table = ProbabilityTable::from_reader(reader, 3)?;
    ///
    /// println!("{:?}", table.generate_words(15)); // Generate 15 word
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate_words(&self, amount: u32) -> Vec<String> {
        generate_multiple_words(self, amount)
    }
}

// Replace each new line characters by a series of space of length
fn add_space(reader: impl BufRead, length: usize) -> io::Result<String> {
    reader
        .lines()
        .map(|line| -> io::Result<String> {
            Ok(format!("{}{}", " ".repeat(length), line?.to_lowercase()))
        })
        .collect::<io::Result<String>>()
}

// Generate a ProbabilityTable from the output of add_space
fn generate_table(spaced_file: String, input_length: usize) -> ProbabilityTable {
    let mut table = ProbabilityTable::new(input_length);
    let chars_list: Vec<_> = spaced_file.chars().collect();
    for charactere in 0..chars_list.len() - input_length {
        let key: String = chars_list
            .get(charactere..charactere + input_length)
            .unwrap()
            .iter()
            .collect();

        let value: char = *chars_list.get(charactere + input_length).unwrap();

        *table
            .table
            .entry(key)
            .or_default()
            .entry(value)
            .or_default() += 1;
    }
    table
}

// Generate one word from a ProbabilityTable
fn generate_word(table: &ProbabilityTable, rng: &mut ThreadRng) -> String {
    let mut out = " ".repeat(table.input_length);
    loop {
        let chars_list: Vec<_> = out.chars().collect();
        let key = &chars_list[chars_list.len() - table.input_length..]
            .iter()
            .collect::<String>();
        let choices = table.table.get(key).unwrap();
        let weight = WeightedIndex::new(choices.values()).unwrap();
        let next_letter = choices.keys().collect::<Vec<&char>>()[weight.sample(rng)];
        out += &next_letter.to_string();
        if out.ends_with(' ') {
            break;
        }
    }
    out.trim().to_string()
}

fn generate_multiple_words(matrix: &ProbabilityTable, number: u32) -> Vec<String> {
    let mut vec_string = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..number {
        vec_string.push(generate_word(&matrix, &mut rng));
    }
    vec_string
}

/// Genrate words that souds like those from a language by analyzing it
/// (from a reader containing each word on a different line), with the
/// precision of `input_length` (how many letters are take in consideration when
/// generating the next letter) and the number of word to generate

/// Generate `amount` word with choosable `accuracy` by analyzing a language
/// under the form of a type that implement [`BufRead`]
pub fn generate_words(
    reader: impl BufRead,
    accuracy: usize,
    amout: u32,
) -> io::Result<Vec<String>> {
    let mut out = generate_multiple_words(
        &generate_table(add_space(reader, accuracy)?, accuracy),
        amout,
    );
    out.sort_by_key(|a| a.len());
    Ok(out)
}
