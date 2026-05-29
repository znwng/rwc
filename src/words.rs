use std::fs::File;
use std::io::{self, Read};

pub fn get_file_word_count(file_path: &str) -> io::Result<usize> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(count_words(&buffer))
}

pub fn get_file_word_count_stdin() -> io::Result<usize> {
    let mut buffer = Vec::new();

    io::stdin().read_to_end(&mut buffer)?;

    Ok(count_words(&buffer))
}

fn count_words(buffer: &[u8]) -> usize {
    let text = String::from_utf8_lossy(buffer);
    let mut in_word = false;
    let mut count = 0;

    for ch in text.chars() {
        if ch.is_whitespace() {
            in_word = false;
        } else if !in_word {
            count += 1;
            in_word = true;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::count_words;

    #[test]
    fn counts_words_separated_by_unicode_whitespace() {
        assert_eq!(count_words("a\u{00a0}b\n".as_bytes()), 2);
    }

    #[test]
    fn treats_invalid_utf8_as_word_content() {
        assert_eq!(count_words(b"a \xff b\n"), 3);
    }
}
