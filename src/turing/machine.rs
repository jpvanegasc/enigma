use crate::turing::states::Direction;
use crate::turing::states::State;

pub struct Machine {
    pub tape: Vec<i32>,
    pub head: usize,
    pub current_state: usize,
    pub states: Vec<State>,
}

impl Machine {
    pub fn new(tape: Vec<i32>, states: Vec<State>) -> Machine {
        Machine {
            tape,
            head: 0,
            current_state: 0,
            states,
        }
    }

    pub fn run(&mut self) {
        loop {
            let current_value = self.tape[self.head];
            let state = &self.states[self.current_state];
            let operation = state.operations.get(&current_value).unwrap();

            if let Some(write) = operation.write {
                self.tape[self.head] = write;
            }

            if let Some(move_head) = operation.move_head {
                match move_head {
                    Direction::Left => {
                        self.head -= 1;
                    }
                    Direction::Right => {
                        self.head += 1;
                    }
                }
            }

            if let Some(next_state) = operation.next_state {
                if next_state == -1 {
                    break;
                }
                self.current_state = next_state as usize;
            }
        }
    }
}
