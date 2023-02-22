use std::error::Error;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename);
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("1.0")
        .author("hend0x")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
            )
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .help("number lines")
            .takes_value(false)
            .conflicts_with("numer_nonblank")
            )
        .arg(
            Arg::with_name("number_nonblank")
            .short("b")
            .long("number-nonblank")
            .takes_value(false)
            )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
