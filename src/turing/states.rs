use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct State {
    pub identifier: i32,
    pub operations: HashMap<i32, StateOperation>,
}

pub struct StateOperation {
    pub write: Option<i32>,
    pub move_head: Option<Direction>,
    pub next_state: Option<i32>,
}
