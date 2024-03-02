use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

// usize
//   ポインタサイズの符号なし整数型
//   32ビットOSなら 4バイト
//   64ビットOSなら 8バイト
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matchs = App::new("headr")
        .version("0.1.0")
        .author("takahiro izumikawa")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Number of lines")
                .takes_value(true)
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("Number of bytes")
                .takes_value(true)
                .conflicts_with("lines")
        )
        .get_matches();

    Ok(
        Config {
            files: matchs.values_of_lossy("files").unwrap(),
            lines: parse_positive_int(&matchs.value_of_lossy("lines").unwrap()).unwrap(),
            bytes: parse_positive_int(&matchs.value_of_lossy("bytes").unwrap()).ok(),
        }
    )
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into())
        // _ => Err(From::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
    
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());

    let res = parse_positive_int("-11");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "-11".to_string());
}