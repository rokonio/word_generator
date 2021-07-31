use word_generator::{langs, *};

use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(langs::fr_txt());
    let accuracy = 3;
    let number_of_words = 15;
    println!("{:#?}", generate_words(reader, accuracy, number_of_words)?);
    Ok(())
}
