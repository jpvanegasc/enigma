use std::collections::HashMap;

use crate::turing::machine::Machine;
use crate::turing::states::build_binary_state;
use crate::turing::states::Direction;
use crate::turing::states::StateOperation;

pub mod turing;

fn main() {
    let q0 = build_binary_state(
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some("0".to_string()),
        },
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some("1".to_string()),
        },
    );
    let q1 = build_binary_state(
        StateOperation {
            write: Some(1),
            move_head: Some(Direction::Left),
            next_state: Some("2".to_string()),
        },
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some("1".to_string()),
        },
    );
    let q2 = build_binary_state(
        StateOperation {
            write: None,
            move_head: Some(Direction::Right),
            next_state: Some("3".to_string()),
        },
        StateOperation {
            write: None,
            move_head: Some(Direction::Left),
            next_state: Some("2".to_string()),
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
            next_state: Some("halt".to_string()),
        },
    );

    let mut machine = Machine::new(
        vec![0, 1, 1, 0, 1, 1, 1],
        {
            let mut states = HashMap::new();
            states.insert("0".to_string(), q0);
            states.insert("1".to_string(), q1);
            states.insert("2".to_string(), q2);
            states.insert("3".to_string(), q3);
            states
        },
        "0".to_string(),
    );
    machine.run();
    println!("{:?}", machine.tape);
    println!("halted")
}
