extern crate regex;

use crate::interpreter::tokenizer::parse_file;
use std::env;
use std::process;

pub mod common;
pub mod interpreter;
pub mod turing;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    let tokens = {
        match parse_file(filename) {
            Ok(tokens) => tokens,
            Err(e) => panic!("Error parsing file: {}", e),
        }
    };

    println!("{:?}", tokens);
}
