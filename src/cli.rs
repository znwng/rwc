use std::env;

use crate::bytes::{get_file_bytes, get_file_bytes_stdin};

use crate::lines::{get_file_line_count, get_file_line_count_stdin};

use crate::maxline::{get_max_line_length, get_max_line_length_stdin};

use crate::words::{get_file_word_count, get_file_word_count_stdin};

pub fn print_help() {
    println!(
        r"rwc - simple Rust word count utility

USAGE:
    rwc [FLAG] [FILE]

FLAGS:
    -l      Count lines
    -w      Count words
    -b      Count bytes
    -L      Get max line length

INPUT MODES:
    FILE    Provide a file path
    STDIN   Pipe input if no file is given
"
    );
}

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let (flag, file_opt) = if args.len() == 2 {
        (args[1].as_str(), None)
    } else if args.len() >= 3 {
        (args[1].as_str(), Some(args[2].as_str()))
    } else {
        print_help();
        return;
    };

    match (flag, file_opt) {
        ("-l", Some(file)) => {
            println!("{}", get_file_line_count(file).unwrap())
        }

        ("-l", None) => {
            println!("{}", get_file_line_count_stdin().unwrap())
        }

        ("-w", Some(file)) => {
            println!("{}", get_file_word_count(file).unwrap())
        }

        ("-w", None) => {
            println!("{}", get_file_word_count_stdin().unwrap())
        }

        ("-b", Some(file)) => {
            println!("{}", get_file_bytes(file).unwrap())
        }

        ("-b", None) => {
            println!("{}", get_file_bytes_stdin().unwrap())
        }

        ("-L", Some(file)) => {
            println!("{}", get_max_line_length(file).unwrap())
        }

        ("-L", None) => {
            println!("{}", get_max_line_length_stdin().unwrap())
        }

        ("-h", _) => print_help(),

        _ => eprintln!("Unknown flag"),
    }
}

