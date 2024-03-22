use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!("{}{}{}{}{}",
                             format_field(info.num_lines, config.lines),
                             format_field(info.num_words, config.words),
                             format_field(info.num_bytes, config.bytes),
                             format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                }
            }
        }
    }
    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:8}", value)
    } else {
        "".to_string()
    }
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    // 行数表示有無
    lines: bool,
    // 単語数表示有無
    words: bool,
    // 単語数表示有無
    bytes: bool,
    // 文字数表示有無
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();

    loop {
        // line に読み込み、読み込みバイト数が返される
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("izumix03")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("bytes")
                .long("bytes")
                .short("c")
                .takes_value(false)
                .help("Show byte count")
        )
        .arg(
            Arg::with_name("chars")
                .long("chars")
                .short("m")
                .takes_value(false)
                .conflicts_with("bytes")
                .help("Show character count")
        )
        .arg(
            Arg::with_name("lines")
                .long("lines")
                .takes_value(false)
                .short("l")
                .help("show line count")
        )
        .arg(
            Arg::with_name("words")
                .long("words")
                .short("w")
                .takes_value(false)
                .help("Show word count")
        ).get_matches();

    let mut lines: bool = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    // 何も指定していない場合は下記3つは true
    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{count, FileInfo, format_field};

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }


    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}