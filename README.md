# 🧠 Rustfuck. A Brainfuck Interpreter
This is a basic Brainfuck interpreter implemented in Rust.

## What is Brainfuck?

Brainfuck is an esoteric programming language created in 1993 by Urban Müller. It is known for its extreme minimalism and is designed to challenge and amuse programmers. The language consists of only eight commands, making it Turing complete, yet difficult to program in due to its simplistic nature.

### Commands

Brainfuck uses a very small set of commands, each represented by a single character:

- `>` : Increment the data pointer (move to the next cell to the right).
- `<` : Decrement the data pointer (move to the next cell to the left).
- `+` : Increment the byte at the data pointer.
- `-` : Decrement the byte at the data pointer.
- `.` : Output the byte at the data pointer.
- `,` : Input a byte and store it in the cell at the data pointer.
- `[` : Jump forward to the command after the matching `]` if the byte at the data pointer is zero.
- `]` : Jump back to the command after the matching `[` if the byte at the data pointer is nonzero.


## Features
- Interprets Brainfuck code input via standard input.
- Supports the standard Brainfuck commands.
- Detects and reports syntax errors, such as mismatched brackets.

## Prerequisites
- Rust (stable) installed. If not, download and install Rust from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

Clone the repository:
```bash
git clone https://github.com/sa4dus/rustfuck.git
cd rustfuck
```
## Usage
To run the interpreter, use Cargo (Rust's package manager):
```bash
cargo run --release -- <file_name.b>
```
Replace `<file_name.b>` with the path to your Brainfuck code file.

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

This project was inspired by various Brainfuck resources and archives, especially **The Brainfuck Archive** by Panu Kalliokoski. Panu's work has been invaluable in collecting and preserving Brainfuck programs, implementations, and utilities. You can find the archive [here](https://sange.fi/esoteric/brainfuck/) for more information and resources.