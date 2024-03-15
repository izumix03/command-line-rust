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

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");
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

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}