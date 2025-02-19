use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wc")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Show word count")
                .takes_value(false)
                .short("w")
                .long("words"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Show byte count")
                .takes_value(false)
                .short("c")
                .long("bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Show character count")
                .takes_value(false)
                .short("m")
                .long("chars")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Show line count")
                .takes_value(false)
                .short("l")
                .long("lines"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if vec![words, bytes, chars, lines].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match count(&filename) {
            Ok(info) => {
                println!(
                    "{}{}{}{}{}",
                    format_field(info.num_lines, config.lines),
                    format_field(info.num_words, config.words),
                    format_field(info.num_bytes, config.bytes),
                    format_field(info.num_chars, config.chars),
                    if filename == &"-".to_string() {
                        "".to_string()
                    } else {
                        format!(" {}", &filename)
                    },
                );
                total_lines += info.num_lines;
                total_words += info.num_words;
                total_bytes += info.num_bytes;
                total_chars += info.num_chars;
            }
            Err(err) => eprintln!("{}: {}", &filename, err),
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

// --------------------------------------------------
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:8}", value)
    } else {
        "".to_string()
    }
}

// --------------------------------------------------
pub fn count(filename: &str) -> MyResult<FileInfo> {
    let mut file: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    };
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line
            .split_whitespace()
            .into_iter()
            .collect::<Vec<&str>>()
            .len();
        num_chars += line.chars().collect::<Vec<char>>().len();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}
