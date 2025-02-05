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
    pub next_state: Option<String>,
}

pub fn build_binary_state(
    zero_state_operation: StateOperation,
    one_state_operation: StateOperation,
) -> State {
    let mut operations = HashMap::new();
    operations.insert(0, zero_state_operation);
    operations.insert(1, one_state_operation);
    State { operations }
}
