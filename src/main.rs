use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    match fs::read_to_string(filename) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error reading file {}: {}", filename, e),
    }
}
