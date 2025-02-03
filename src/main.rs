use std::collections::HashMap;

enum Direction {
    Left,
    Right,
}

struct State {
    identifier: i32,
    operations: HashMap<i32, StateOperation>,
}

struct StateOperation {
    write: Option<i32>,
    move_head: Option<Direction>,
    next_state: Option<i32>,
}

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
                    move_head: Some(Direction::Right),
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
}
