//use clap::{Arg, ArgAction, Command};
use anyhow::Result;
//use predicates::prelude::*;
//use pretty_assertions::assert_eq;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        println!("{filename}")
    }
    Ok(())
}

fn open(filename: &str) ->Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    //let args = Args::parse();
    //println!("{args:#?}");
    if let Err(e) = run(Args::parse()) {
        eprint!("{e}");
        std::process::exit(1);
    }
}
