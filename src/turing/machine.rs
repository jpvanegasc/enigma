use crate::turing::states::Direction;
use crate::turing::states::State;
use std::collections::HashMap;

pub struct Machine {
    pub tape: Vec<i32>,
    pub head: usize,
    pub current_state: String,
    pub states: HashMap<String, State>,
}

impl Machine {
    pub fn new(tape: Vec<i32>, states: HashMap<String, State>, start_state: String) -> Machine {
        Machine {
            tape,
            head: 0,
            current_state: start_state,
            states,
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
                    Direction::Left => {
                        self.head -= 1;
                    }
                    Direction::Right => {
                        self.head += 1;
                    }
                }
            }

            if operation.next_state.is_some() {
                let next_state = operation.next_state.as_ref().unwrap();
                if next_state == "halt" {
                    break;
                }
                self.current_state = String::from(next_state);
            }
        }
    }
}
