use std::env::Args;
use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

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
        ).args(&*vec![
        Arg::with_name("bytes")
            .long("bytes")
            .short("c")
            .help("Show byte count"),
        Arg::with_name("chars")
            .long("chars")
            .short("m")
            .conflicts_with("bytes")
            .help("Show character count"),
        Arg::with_name("lines")
            .long("lines")
            .short("l")
            .help("show line count"),
        Arg::with_name("words")
            .long("words")
            .short("w")
            .help("Show word count"),
    ]).get_matches();

    let lines = matches.is_present("lines");
    let words = matches.is_present("words");
    let bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: if lines {true} else if words || bytes || chars {false} else {true},
        words: if words {true} else if lines || bytes || chars {false} else {true},
        bytes: if bytes {true} else if lines || words || chars {false} else {true},
        chars: if chars {true} else if lines || words || bytes {false} else {false},
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}