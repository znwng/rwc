use std::fs::File;
use std::io::{self, Read};

pub fn get_file_bytes(file_path: &str) -> io::Result<usize> {
    let mut file = File::open(file_path)?;

    let mut buffer = [0u8; 32768];
    let mut count = 0;

    loop {
        let n = file.read(&mut buffer)?;

        if n == 0 {
            break;
        }

        count += n;
    }

    Ok(count)
}

pub fn get_file_bytes_stdin() -> io::Result<usize> {
    let mut buffer = [0u8; 32768];
    let mut count = 0;

    loop {
        let n = io::stdin().read(&mut buffer)?;

        if n == 0 {
            break;
        }

        count += n;
    }

    Ok(count)
}
