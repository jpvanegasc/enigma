use crate::common::Direction;
use crate::common::State;
use crate::common::StateOperation;
use crate::interpreter::parser_old::Program;
use crate::turing::Machine;
use std::collections::HashMap;

pub fn build_machine(program: Program) -> Machine {
    let tape = {
        let mut tape = Vec::new();
        for c in program.tape.split_whitespace() {
            tape.push(c.parse::<i32>().unwrap());
        }
        tape
    };
    let states = {
        let mut states = HashMap::new();
        for state in program.states {
            let mut operations = HashMap::new();
            for line in state.lines {
                let (input, instructions) = line.split_once("->").unwrap();
                let parts: Vec<&str> = instructions.split(",").map(|s| s.trim()).collect();

                let write = if parts[0] != "null" {
                    Some(
                        parts[0]
                            .split_whitespace()
                            .last()
                            .unwrap()
                            .parse::<i32>()
                            .unwrap(),
                    )
                } else {
                    None
                };

                let move_head = if parts[1] != "null" {
                    Some(match parts[1].split_whitespace().last().unwrap() {
                        "left" => Direction::Left,
                        "right" => Direction::Right,
                        _ => panic!("Invalid direction"),
                    })
                } else {
                    None
                };

                let next_state = if parts[2] != "null" {
                    Some(parts[2].split_whitespace().last().unwrap().to_string())
                } else {
                    None
                };

                operations.insert(
                    input.trim().parse::<i32>().unwrap(),
                    StateOperation {
                        write,
                        move_head,
                        next_state,
                    },
                );
            }
            states.insert(state.identifier, State { operations });
        }
        states
    };
    Machine::new(tape, states, program.initial_state)
}
