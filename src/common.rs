use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct State {
    pub operations: HashMap<i32, StateOperation>,
}

#[derive(Debug)]
pub struct StateOperation {
    pub write: Option<i32>,
    pub move_head: Option<Direction>,
    pub next_state: String,
}

impl StateOperation {
    pub fn new(
        write: Option<i32>,
        move_head: Option<Direction>,
        next_state: String,
    ) -> StateOperation {
        StateOperation {
            write,
            move_head,
            next_state,
        }
    }
}

pub fn build_binary_state(
    zero_state_operation: StateOperation,
    one_state_operation: StateOperation,
) -> State {
    let operations = HashMap::from([(0, zero_state_operation), (1, one_state_operation)]);
    State { operations }
}
