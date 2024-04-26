use std::error::Error;
use clap::{App, Arg};
use walkdir::{DirEntry, WalkDir};
use regex::Regex;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>, // ファイル名やディレクトリ名
    names: Vec<Regex>, // 正規表現
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("izumix")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Search paths")
                .multiple(true)
                .default_value(".")
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .help("Name")
                .long("name")
                .short("n")
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("type")
                .value_name("TYPE")
                .help("Entry type")
                .short("t")
                .long("type")
                .possible_values(&["f", "d", "l"])
                .takes_value(true)
                .multiple(true)
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of("type")
        .map(parse_entry_type)
        .transpose()?
        .into_iter().flatten()
        .collect();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = | entry: &DirEntry | {
        config.entry_types.is_empty() ||
            config
                .entry_types
                .iter()
                .any(| entry_type | match entry_type {
                    EntryType::Link => entry.file_type().is_symlink(),
                    EntryType::Dir => entry.file_type().is_dir(),
                    EntryType::File => entry.file_type().is_file(),
                })
    };

    let name_filter = |entry: &DirEntry | {
        config.names.is_empty()
        || config
            .names
            .iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}

fn parse_entry_type(vals: clap::Values) -> MyResult<Vec<EntryType>> {
    vals.map(|v| {
        match v {
            "f" => Ok(EntryType::File),
            "d" => Ok(EntryType::Dir),
            "l" => Ok(EntryType::Link),
            _ => {
                format!("illegal type -- {}", v);
                Err(v.into())
            }
        }
    }).collect()
}