use crate::interpreter::builder::build_machine;
use crate::interpreter::parser::parse_file;

pub mod interpreter;
pub mod turing;

fn main() {
    let program = {
        match parse_file("examples/unary-addition.en") {
            Ok(program) => program,
            Err(e) => panic!("Error parsing file: {}", e),
        }
    };

    let mut machine = build_machine(program);
    machine.run();
    println!("{:?}", machine.tape);
}
