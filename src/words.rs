use std::fs::File;
use std::io::{self, Read};

pub fn get_file_word_count(file_path: &str) -> io::Result<usize> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    let mut in_word = false;
    let mut count = 0;

    for &byte in &buffer {
        if byte.is_ascii_whitespace() {
            in_word = false;
        } else if !in_word {
            count += 1;
            in_word = true;
        }
    }

    Ok(count)
}

pub fn get_file_word_count_stdin() -> io::Result<usize> {
    let mut buffer = Vec::new();

    io::stdin().read_to_end(&mut buffer)?;

    let mut in_word = false;
    let mut count = 0;

    for &byte in &buffer {
        if byte.is_ascii_whitespace() {
            in_word = false;
        } else if !in_word {
            count += 1;
            in_word = true;
        }
    }

    Ok(count)
}

