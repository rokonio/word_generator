//! Utilities for generating word
//!
//! Generate word that souds like word from a language.
//! The language must be a [`BufReader`] that has one word per line
//!
//! # Usage
//!

use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

// The matrix of chance (u32) of a lettre (char) to appear
// after a sequence of lettres (String) of size state_size
#[derive(Debug)]
pub struct GeneratedMatrix {
    matrix: HashMap<String, HashMap<char, u32>>,
    state_size: usize,
}

impl GeneratedMatrix {
    fn new(state_size: usize) -> GeneratedMatrix {
        GeneratedMatrix {
            matrix: HashMap::new(),
            state_size,
        }
    }

    pub fn from_reader(reader: impl BufRead, state_size: usize) -> io::Result<GeneratedMatrix> {
        Ok(generate_matrix(add_space(reader, state_size)?, state_size))
    }

    pub fn generate_words(&self, number: u32) -> Vec<String> {
        generate_multiple_words(self, number)
    }
}

// Add space of size state_size beetween each word (1 word per line)
fn add_space(reader: impl BufRead, state_size: usize) -> io::Result<String> {
    reader
        .lines()
        .map(|line| -> io::Result<String> { Ok(format!("{}{}", " ".repeat(state_size), line?)) })
        .collect::<io::Result<String>>()
}

// Generate the matrix from the output of add_space
fn generate_matrix(spaced_file: String, state_size: usize) -> GeneratedMatrix {
    let mut matrix = GeneratedMatrix::new(state_size);
    let chars_list: Vec<_> = spaced_file.chars().collect();
    for charactere in 0..chars_list.len() - state_size {
        let key: String = chars_list
            .get(charactere..charactere + state_size)
            .unwrap()
            .iter()
            .collect();
        let value: char = *chars_list.get(charactere + state_size).unwrap();

        let counter = matrix.matrix.entry(key).or_default();
        let ccounter = counter.entry(value).or_insert(0);
        *ccounter += 1;
    }
    matrix
}

// Generate one word from the matrix
fn generate_word(matrix: &GeneratedMatrix) -> String {
    let mut out = " ".repeat(matrix.state_size);
    let mut rng = thread_rng();
    loop {
        let chars_list: Vec<_> = out.chars().collect();
        let choices = matrix
            .matrix
            .get(
                &chars_list[chars_list.len() - matrix.state_size..]
                    .iter()
                    .collect::<String>(),
            )
            .unwrap();
        let next_letter =
            choices.keys().collect::<Vec<&char>>()[WeightedIndex::new(choices.values())
                .unwrap()
                .sample(&mut rng)];
        out += &next_letter.to_string();
        if out.ends_with(' ') {
            break;
        }
    }
    out.trim().to_string()
}

fn generate_multiple_words(matrix: &GeneratedMatrix, number: u32) -> Vec<String> {
    let mut vec_string = Vec::new();
    for _ in 0..number {
        vec_string.push(generate_word(&matrix))
    }
    vec_string
}

/// Genrate words that souds like those from a language by analyzing it
/// (from a reader containing each word on a different line), with the
/// precision of `state_size` (how many letters are take in consideration when
/// generating the next letter) and the number of word to generate
pub fn generate_words(
    reader: impl BufRead,
    state_size: usize,
    number_of_words: u32,
) -> io::Result<Vec<String>> {
    let mut out = generate_multiple_words(
        &generate_matrix(add_space(reader, state_size)?, state_size),
        number_of_words,
    );
    out.sort_by_key(|a| a.len());
    // out.sort_by_key();
    Ok(out)
}
