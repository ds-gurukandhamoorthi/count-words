use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let filename  = env::args().skip(1).next().unwrap();
    let mut words_count = HashMap::new();

    let input = File::open(filename)?;

    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let line = line?;
        for word in line.split(|c| char::is_whitespace(c) || char::is_ascii_punctuation(&c) || char::is_ascii_digit(&c)){
            *words_count.entry(String::from(word)).or_insert(0_u32) += 1;
        }
    }

    println!("{},{}", "word", "count");
    for (word, count) in words_count {
        println!("{},{}", word, count);
    }

    // println!("{:?}", words_count);

    Ok(())
}
