use crate::common::Direction::*;
use crate::common::State;
use std::collections::HashMap;

pub struct Machine {
    pub tape: Vec<i32>,
    pub head: usize,
    pub current_state: String,
    pub states: HashMap<String, State>,
    pub steps: u64,
}

impl Machine {
    pub fn new(tape: Vec<i32>, states: HashMap<String, State>, start_state: String) -> Machine {
        Machine {
            tape,
            head: 0,
            current_state: start_state,
            states,
            steps: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let current_value = self.tape[self.head];
            let state = &self.states[&self.current_state.to_string()];
            let operation = state.operations.get(&current_value).unwrap();

            if operation.write.is_some() {
                self.tape[self.head] = operation.write.unwrap();
            }

            if operation.move_head.is_some() {
                match operation.move_head.unwrap() {
                    Left => {
                        self.head -= 1;
                    }
                    Right => {
                        self.head += 1;
                    }
                }
            }

            self.current_state = String::from(&operation.next_state);
            if self.current_state == "halt" {
                break;
            }

            self.steps += 1;
        }
    }

    pub fn current_status(&self) {
        if self.tape.len() > 20 {
            println!("Tape is to big to display, sorry");
            return;
        }

        println!("state: '{0}', steps: {1}", self.current_state, self.steps);

        print!("[ ");
        for x in &self.tape {
            print!("{x} ");
        }
        print!("]\n  ");

        for i in 0..self.tape.len() {
            if i == self.head {
                print!("^ ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
