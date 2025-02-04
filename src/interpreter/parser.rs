use std::fs;

#[derive(Debug)]
pub struct RawState {
    identifier: String,
    lines: Vec<String>,
}

pub struct Program {
    pub tape: String,
    pub initial_state: String,
    pub states: Vec<RawState>,
}

fn get_program(file_contents: &str) -> Program {
    let mut tape = String::new();
    let mut initial_state = String::new();
    let mut states = Vec::new();
    let mut temp_state = RawState {
        identifier: String::new(),
        lines: Vec::new(),
    };
    let mut active_state = false;
    for line in file_contents.lines() {
        if line.starts_with("start:") {
            initial_state = line.split_whitespace().last().unwrap().to_string();
        } else if line.starts_with("state") {
            temp_state.identifier = line.split_whitespace().last().unwrap().to_string();
            active_state = true;
        } else if active_state & line.starts_with("    ") {
            temp_state.lines.push(line.to_string());
        } else if active_state {
            states.push(temp_state);
            temp_state = RawState {
                identifier: String::new(),
                lines: Vec::new(),
            };
            active_state = false;
        } else if line.starts_with("initial") {
            tape = line.to_string();
        }
    }
    Program {
        tape,
        initial_state,
        states,
    }
}

pub fn parse_file(file_path: &str) -> Program {
    match fs::read_to_string(file_path) {
        Ok(contents) => get_program(&contents),
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            Program {
                tape: String::new(),
                initial_state: String::new(),
                states: Vec::new(),
            }
        }
    }
}
