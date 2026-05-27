use std::fs::File;
use std::io::{self, BufRead, BufReader};

use unicode_width::UnicodeWidthStr;

pub fn get_max_line_length(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut max = 0;

    for line in reader.lines() {
        let line = line?;
        let len = UnicodeWidthStr::width(line.as_str());

        if len > max {
            max = len;
        }
    }

    Ok(max)
}

pub fn get_max_line_length_stdin() -> io::Result<usize> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut max = 0;

    for line in reader.lines() {
        let line = line?;
        let len = UnicodeWidthStr::width(line.as_str());

        if len > max {
            max = len;
        }
    }

    Ok(max)
}

