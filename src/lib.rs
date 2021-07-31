// By Rokonio <naelkanada+rust@gmail.com>
// This crate is under the MIT liscence <LISCENCE or
// https://opensource.org/licenses/MIT>

//! Utilities for generating random words that souds similar to a choosen
//! language.
//!
//! Word_generator provides utilities for generating words that souds similar
//! to a choosen language.
//! It uses
//! [Markov Chains](https://en.wikipedia.org/wiki/Markov_chain#Examples) under
//! the name of `ProbabilityTable` to analyze the likehood of each characters
//! to appears after the nths previous characters where n is variable. n can be consider
//! as the accuracy
//!
//! This idea came from a
//![Science Étonnante's video (in french)](https://youtu.be/YsR7r2378j0)
//!
//! # Example
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use std::{fs::File, io::BufReader};
//! use word_generator::{langs, *};
//!
//! // let reader = BufReader::new(File::open("Your_lang.txt")?); // using your language
//! let reader = BufReader::new(langs::fr_txt()); // or a preexisting language
//!
//! // This
//! let table = ProbabilityTable::from_reader(reader, 3)?;
//! println!("{:?}", table.generate_words(15)); // Generate 15 word
//!
//! # let reader = BufReader::new(langs::fr_txt());
//!
//! // Is the same as this
//! println!("{:?}", generate_words(reader, 3, 15)?);
//! # Ok(())
//! # }
//! ```
//!
//! # Licences
//!
//! Here is the list of licences of the languages on this crate:
//!  - French: free of use
//!
//! If you have more language to add please submit a PR at
//! [the GitHub of this project](https://github.com/rokonio/word_generator)
use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub mod langs;
/// This is a
/// [Markov Chains](https://en.wikipedia.org/wiki/Markov_chain#Examples) under
/// the name of `ProbabilityTable`. It represents the likehood of each
/// characters of the language to appears after the nths previous caracters where n is
/// variable. n can be consider as the accuracy
///
/// # Example
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// use std::io::BufReader;
/// use word_generator::{langs, *};
///
/// let reader = BufReader::new(langs::fr_txt());
///
/// let table = ProbabilityTable::from_reader(reader, 3)?;
/// println!("{:?}", table.generate_words(15)); // Generate 15 word
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ProbabilityTable {
    pub(crate) table: HashMap<String, HashMap<char, u32>>,
    pub(crate) accuracy: usize,
}

impl ProbabilityTable {
    fn new(accuracy: usize) -> ProbabilityTable {
        ProbabilityTable {
            table: HashMap::new(),
            accuracy,
        }
    }

    /// Construct a new `ProbabilityTable` from reader. It can be any type that
    /// implements [`BufRead`].
    ///
    /// # Panic
    ///
    /// Panic if `accuracy` is less than one.
    pub fn from_reader(reader: impl BufRead, accuracy: usize) -> io::Result<ProbabilityTable> {
        assert!(accuracy >= 1);
        Ok(generate_table(add_space(reader, accuracy)?, accuracy))
    }

    /// Generate `amount` words.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// use std::io::BufReader;
    /// use word_generator::{langs, *};
    ///
    /// let reader = BufReader::new(langs::fr_txt());
    ///
    /// let table = ProbabilityTable::from_reader(reader, 3)?;
    /// println!("{:?}", table.generate_words(15)); // Generate 15 word
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate_words(&self, amount: u32) -> Vec<String> {
        generate_multiple_words(self, amount)
    }
}

// Replace each new line characters by a series of space of length
fn add_space(reader: impl BufRead, accuracy: usize) -> io::Result<String> {
    reader
        .lines()
        .map(|line| -> io::Result<String> {
            Ok(format!("{}{}", " ".repeat(accuracy), line?.to_lowercase()))
        })
        .collect::<io::Result<String>>()
}

// Generate a ProbabilityTable from the output of add_space
fn generate_table(spaced_file: String, accuracy: usize) -> ProbabilityTable {
    let mut table = ProbabilityTable::new(accuracy);
    let chars_list: Vec<_> = spaced_file.chars().collect();
    for charactere in 0..chars_list.len() - accuracy {
        let key: String = chars_list
            .get(charactere..charactere + accuracy)
            .unwrap()
            .iter()
            .collect();

        let value: char = *chars_list.get(charactere + accuracy).unwrap();

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
    let mut out = " ".repeat(table.accuracy);
    loop {
        let chars_list: Vec<_> = out.chars().collect();
        let key = &chars_list[chars_list.len() - table.accuracy..]
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

/// Generate `amount` word with choosable `accuracy` by analyzing a language
/// under the form of a type that implement [`BufRead`]
///
/// # Example
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// use std::io::BufReader;
/// use word_generator::{langs, *};
///
/// let reader = BufReader::new(langs::fr_txt());
///
/// println!("{:?}", generate_words(reader, 3, 15)?);
/// # Ok(())
/// # }
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufReader};

    #[test]
    fn table_generate_words_works() -> io::Result<()> {
        let reader = BufReader::new(langs::fr_txt());
        let table = ProbabilityTable::from_reader(reader, 3)?;
        assert_eq!(table.generate_words(15).len(), 15);
        Ok(())
    }

    #[test]
    fn table_consticency() -> io::Result<()> {
        let reader = BufReader::new(langs::fr_txt());
        let table = ProbabilityTable::from_reader(reader, 3)?;
        for _ in 0..100 {
            assert_eq!(
                table
                    .table
                    .keys()
                    .nth(random::<u8>().into())
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>()
                    .len(),
                3
            );
        }
        Ok(())
    }
}
