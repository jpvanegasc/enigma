use std::collections::HashMap;

use crate::turing::machine::Machine;
use crate::turing::states::build_binary_state;
use crate::turing::states::Direction;
use crate::turing::states::State;
use crate::turing::states::StateOperation;

pub mod turing;

fn main() {
    let halt = State {
        identifier: -1,
        operations: HashMap::new(),
    };
    let q0 = build_binary_state(
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some(0),
        },
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some(1),
        },
    );
    let q1 = build_binary_state(
        StateOperation {
            write: Some(1),
            move_head: Some(Direction::Left),
            next_state: Some(2),
        },
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some(1),
        },
    );
    let q2 = build_binary_state(
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some(3),
        },
        StateOperation {
            write: None,
            move_head: Some(Direction::Left),
            next_state: Some(2),
        },
    );
    let q3 = build_binary_state(
        StateOperation {
            write: None,
            move_head: None,
            next_state: None,
        },
        StateOperation {
            write: Some(0),
            move_head: None,
            next_state: Some(-1),
        },
    );

    let mut machine = Machine::new(vec![0, 1, 1, 0, 1, 1, 1], vec![q0, q1, q2, q3, halt]);
    machine.run();
    println!("{:?}", machine.tape);
}
