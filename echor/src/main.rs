// cargo run -- -n hello
// -- をつけることで cargo への引数じゃないことを明示
// cargo run -- --version
// https://github.com/rust-lang/cargo/blob/62837938537f7524f657e4692d672354000fb202/src/bin/cargo/commands/add.rs#L106


// {:#?} は整形表示

use clap::{Arg, App};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();


    let omit_newline = matches.is_present("omit_newline");
    let text = matches.values_of_lossy("text").unwrap();

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
