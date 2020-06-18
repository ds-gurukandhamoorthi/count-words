use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use fnv::{FnvHashMap};

fn main() -> Result<(), Error> {
    let files  = env::args().skip(1);
    let mut words_count = FnvHashMap::default();

    for filename in files {

        let input = File::open(filename)?;

        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            let line = line?;
            for word in line.split(|c| char::is_whitespace(c) || char::is_ascii_punctuation(&c) || char::is_ascii_digit(&c)){
                *words_count.entry(String::from(word)).or_insert(0_u32) += 1;
            }
        }

    }
    println!("{},{}", "word", "count");
    for (word, count) in words_count {
        if !word.is_empty() {
            println!("{},{}", word, count);
        }
    }

    Ok(())
}
