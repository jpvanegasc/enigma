extern crate regex;

use crate::common::Direction;
use crate::common::StateOperation;
use crate::interpreter::parser::get_abstract_syntax_tree;
use crate::interpreter::tokenizer::parse_file;
use std::collections::HashMap;
use std::env;
use std::process;

pub mod common;
pub mod interpreter;
pub mod turing;

/// Temporarily disabled
fn _parse_file() {
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

    let ast = get_abstract_syntax_tree(tokens);
    println!("{:?}", ast);
}

fn main() {
    // let ast = Vec::new();
    // let context_states = Vec::new();
    // let tape_definitions = Vec::new();

    // for top_level in ast {
    //     if top_level.type == StateDef {
    //         context_states.push(top_level.value);
    //     }
    //     if top_level.type == Expression {
    //         if top_level.value == PrintExpr {
    //             // do something
    //         }
    //         let result = turing_run(context_states, top_level.value.initial_state, top_level.value.tape);
    //         tape_definitions.push(result);
    //     }
    // }

    let state = common::build_binary_state(
        StateOperation::new(None, Some(Direction::Right), String::from("q")),
        StateOperation::new(Some(0), None, String::from("halt")),
    );

    let mut machine = turing::Machine::new(
        vec![0, 0, 1],
        HashMap::from([(String::from("q"), state)]),
        String::from("q"),
    );

    machine.current_status();
    machine.run();
    machine.current_status();
}
