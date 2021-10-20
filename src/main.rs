#[macro_use]
extern crate clap;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, AppSettings, Arg, ArgMatches};

const ARG_CHAR: &'static str = "char";
const ARG_DQUOTE: &'static str = "dquote";
const ARG_FILE: &'static str = "file";

fn parse_args(args: &Vec<String>) -> ArgMatches {
    let arg_char = Arg::with_name(ARG_CHAR)
        .help("Quote character")
        .short("c")
        .long(ARG_CHAR)
        .value_name("CHAR");

    let arg_dquote = Arg::with_name(ARG_DQUOTE)
        .help("Use double quote instead of quote")
        .short("d")
        .conflicts_with(ARG_CHAR);

    let arg_input = Arg::with_name(ARG_FILE)
        .help("Input file")
        .short("f")
        .long(ARG_FILE)
        .value_name("FILE");

    return App::new("Quote")
        .version(crate_version!())
        .about("Takes a list of strings and surrounds them with a quoting character")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(&arg_char)
        .arg(&arg_dquote)
        .arg(&arg_input)
        .get_matches_from(args);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = parse_args(&args);
    let char = if matches.is_present(ARG_CHAR) {
        matches.value_of(ARG_CHAR).unwrap()
    } else if matches.is_present(ARG_DQUOTE) {
        "\""
    } else {
        "'"
    };
    let file: Box<dyn BufRead> = match matches.value_of(ARG_FILE) {
        Some(f) => Box::new(BufReader::new(File::open(f).unwrap())),
        None => Box::new(BufReader::new(io::stdin()))
    };

    for line in file.lines().into_iter() {
        println!("{}{}{}", char, line.unwrap(), char);
    }
}

