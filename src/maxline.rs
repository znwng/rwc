use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str;

use unicode_width::UnicodeWidthChar;

pub fn get_max_line_length(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    max_line_length(BufReader::new(file))
}

pub fn get_max_line_length_stdin() -> io::Result<usize> {
    let stdin = io::stdin();
    max_line_length(stdin.lock())
}

fn max_line_length<R: BufRead>(mut reader: R) -> io::Result<usize> {
    let mut max = 0;
    let mut line = Vec::new();

    loop {
        line.clear();
        let bytes_read = reader.read_until(b'\n', &mut line)?;

        if bytes_read == 0 {
            break;
        }

        let len = line_width(strip_line_ending(&line));

        if len > max {
            max = len;
        }
    }

    Ok(max)
}

fn strip_line_ending(mut line: &[u8]) -> &[u8] {
    if line.ends_with(b"\n") {
        line = &line[..line.len() - 1];
    }

    if line.ends_with(b"\r") {
        line = &line[..line.len() - 1];
    }

    line
}

fn line_width(mut bytes: &[u8]) -> usize {
    let mut width = 0;

    while !bytes.is_empty() {
        match str::from_utf8(bytes) {
            Ok(text) => {
                width += text_width(text, width);
                break;
            }
            Err(err) => {
                let valid_up_to = err.valid_up_to();

                if valid_up_to > 0 {
                    let text = str::from_utf8(&bytes[..valid_up_to]).expect("valid UTF-8 prefix");
                    width += text_width(text, width);
                }

                match err.error_len() {
                    Some(error_len) => bytes = &bytes[valid_up_to + error_len..],
                    None => break,
                }
            }
        }
    }

    width
}

fn text_width(text: &str, start_width: usize) -> usize {
    let mut width = 0;

    for ch in text.chars() {
        if ch == '\t' {
            width += 8 - ((start_width + width) % 8);
        } else {
            width += UnicodeWidthChar::width(ch).unwrap_or(0);
        }
    }

    width
}

#[cfg(test)]
mod tests {
    use super::{line_width, max_line_length};
    use std::io::Cursor;

    #[test]
    fn ignores_invalid_utf8_when_measuring_line_width() {
        assert_eq!(line_width(b"\xff\n"), 0);
    }

    #[test]
    fn strips_crlf_line_endings() {
        let input = Cursor::new(b"a\r\nb\n");
        assert_eq!(max_line_length(input).unwrap(), 1);
    }

    #[test]
    fn expands_tabs_to_eight_column_stops() {
        assert_eq!(line_width(b"a\tb"), 9);
    }
}
