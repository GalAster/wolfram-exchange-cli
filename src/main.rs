extern crate clap;
extern crate wolfram_wxf;

pub mod utils;

use crate::utils::{parse_file, write_to_file};
use clap::{App, Arg};

pub enum SupportedFormat {
    Json,
    Toml,
    Yaml,
    Pickle,
}

#[derive(Debug)]
pub enum Error {
    IO
}

fn main() -> Result<(), Error> {
    #[rustfmt::skip]
        let matches = App::new("Wolfram Format Converter")
        .version("0.1.0")
        .author("Aster <192607617@qq.com>")
        .about("Transform yaml, json, pkl files to wolfram")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("Text")
            .short("t")
            .long("text")
            .help("Generates m file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("Binary")
            .short("b")
            .long("binary")
            .help("Generates wxf file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("Compress")
            .short("c")
            .long("compress")
            .help("Generates mx file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("File Format")
            .short("f")
            .long("format")
            .value_name("Format")
            .help("Sets the input file format")
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();
    let value = parse_file(input)?;
    match matches.occurrences_of("Text") {
        0 => (),
        _ => write_to_file(&format!("{}.m", input), value.to_string().as_bytes())?,
    }
    match matches.occurrences_of("Binary") {
        0 => (),
        _ => write_to_file(&format!("{}.wxf", input), &value.to_bytes())?,
    }
    match matches.occurrences_of("Compress") {
        0 => (),
        _ => write_to_file(&format!("{}.mx", input), &value.to_compressed())?,
    };
    Ok(())
}
