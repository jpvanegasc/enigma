use std::collections::HashMap;

use crate::turing::states::Direction;
use crate::turing::states::State;
use crate::turing::states::StateOperation;

pub mod turing;

fn main() {
    let halt = State {
        identifier: -1,
        operations: HashMap::new(),
    };
    let q0 = State {
        identifier: 0,
        operations: HashMap::from([
            (
                0,
                StateOperation {
                    write: None,
                    move_head: Some(Direction::Right),
                    next_state: Some(0),
                },
            ),
            (
                1,
                StateOperation {
                    write: None,
                    move_head: Some(Direction::Right),
                    next_state: Some(1),
                },
            ),
        ]),
    };
    let q1 = State {
        identifier: 1,
        operations: HashMap::from([
            (
                0,
                StateOperation {
                    write: Some(1),
                    move_head: Some(Direction::Left),
                    next_state: Some(2),
                },
            ),
            (
                1,
                StateOperation {
                    write: None,
                    move_head: Some(Direction::Right),
                    next_state: Some(1),
                },
            ),
        ]),
    };
    let q2 = State {
        identifier: 2,
        operations: HashMap::from([
            (
                0,
                StateOperation {
                    write: None,
                    move_head: Some(Direction::Right),
                    next_state: Some(3),
                },
            ),
            (
                1,
                StateOperation {
                    write: None,
                    move_head: Some(Direction::Left),
                    next_state: Some(2),
                },
            ),
        ]),
    };
    let q3 = State {
        identifier: 3,
        operations: HashMap::from([(
            1,
            StateOperation {
                write: Some(0),
                move_head: None,
                next_state: Some(-1),
            },
        )]),
    };

    let mut tape = vec![0, 1, 1, 0, 1, 1, 1];
    let mut head = 0;
    let mut state = &q0;

    loop {
        println!("{:?}", tape);
        println!("head at: {:?}", head);
        println!("current state: {:?}", state.identifier);
        let current_value = tape[head];
        let operation = state.operations.get(&current_value).unwrap();
        if let Some(write) = operation.write {
            tape[head] = write;
        }
        if let Some(move_head) = operation.move_head {
            match move_head {
                Direction::Left => {
                    head -= 1;
                }
                Direction::Right => {
                    head += 1;
                }
            }
        }
        if let Some(next_state) = operation.next_state {
            if next_state == -1 {
                break;
            }
            match next_state {
                0 => state = &q0,
                1 => state = &q1,
                2 => state = &q2,
                3 => state = &q3,
                -1 => state = &halt,
                _ => state = &halt,
            }
        }
    }
    println!("\n{:?}", tape);
    println!("halted");
}
