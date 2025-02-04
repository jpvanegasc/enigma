use crate::interpreter::builder::build_machine;
use crate::interpreter::parser::parse_file;
use std::env;
use std::process;

pub mod interpreter;
pub mod turing;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    let program = {
        match parse_file(filename) {
            Ok(program) => program,
            Err(e) => panic!("Error parsing file: {}", e),
        }
    };

    let mut machine = build_machine(program);
    machine.run();
    println!("{:?}", machine.tape);
}
