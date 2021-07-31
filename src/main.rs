use word_generator::*;

use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("Fr.txt")?);
    let accuracy = 3;
    let number_of_words = 15;
    println!("{:#?}", generate_words(reader, accuracy, number_of_words)?);
    Ok(())
}
