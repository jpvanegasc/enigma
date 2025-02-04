# 🤖 enigma
`enigma` is a simple programming language designed for defining and running Turing machines in a human-readable format. It allows users to easily specify states, transitions, and initial tape contents to simulate computational logic.

## Installation
### Prerequisites
- Rust (Install via [rustup](https://rustup.rs/))

### Clone and Build
```shell
# Clone the repository
git clone https://github.com/your-username/enigma.git
cd enigma

# Build the enigma executable
make
```

## Usage
### Writing an Enigma Program
Create a file, e.g., `program.en`, with the following syntax:
```enigma
# Unary addition calculator

initial: 0 1 1 0 1 1 1
start: q0

state q0
    0 -> null, move right, goto q0
    1 -> null, move right, goto q1

state q1
    0 -> write 1, move left, goto q2
    1 -> null, move right, goto q1

state q2
    0 -> null, move right, goto q3
    1 -> null, move left, goto q2

state q3
    0 -> null, null, null
    1 -> write 0, null, goto halt
```

### Running an Enigma Program
```shell
./enigma program.en
```
This will parse the file and output the computation.
