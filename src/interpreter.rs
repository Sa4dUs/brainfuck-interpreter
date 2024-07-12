use std::io::Read;
use rustc_hash::FxHashMap;

const MEM_SIZE: usize = 1024;

pub struct VirtualMachine {
    cache: FxHashMap<usize, usize>,
    mem: [u8; MEM_SIZE],
}

pub enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

pub trait Interpreter: Lexer + Runnable {
    fn new() -> Self;
    fn exec(&mut self, code: Vec<u8>) -> Vec<u8>;
}

pub trait Lexer {
    fn lex(&mut self, code: Vec<u8>) -> Vec<OpCode>;
}

pub trait Runnable {
    fn run(&mut self, ops: Vec<OpCode>) -> Vec<u8>;
}

impl Interpreter for VirtualMachine {
    fn new() -> Self {
        VirtualMachine {
            mem: [0; MEM_SIZE],
            cache: FxHashMap::default(),
        }
    }

    fn exec(&mut self, code: Vec<u8>) -> Vec<u8> {
        let ops: Vec<OpCode> = self.lex(code);
        let stdout: Vec<u8> = self.run(ops);
        stdout.iter().for_each(|b| print!("{}", *b as char));

        stdout
    }
}

impl Lexer for VirtualMachine {
    fn lex(&mut self, code: Vec<u8>) -> Vec<OpCode> {
        let mut ops: Vec<OpCode> = Vec::new();
        let mut loop_stack: Vec<usize> = Vec::new();
        let mut pc: usize = 0;

        code.iter().for_each(|byte| {
            let op = match byte {
                b'>' => Some(OpCode::IncrementPointer),
                b'<' => Some(OpCode::DecrementPointer),
                b'+' => Some(OpCode::Increment),
                b'-' => Some(OpCode::Decrement),
                b'.' => Some(OpCode::Write),
                b',' => Some(OpCode::Read),
                b'[' => {
                    loop_stack.push(pc);
                    Some(OpCode::LoopBegin)
                }
                b']' => {
                    match loop_stack.pop() {
                        Some(val) => {
                            self.cache.insert(val, pc);
                            self.cache.insert(pc, val);
                        }
                        None => panic!("Syntax Error: unmatched ] at {}", pc),
                    }
                    Some(OpCode::LoopEnd)
                }
                _ => None,
            };

            if let Some(op_code) = op {
                ops.push(op_code);
                pc += 1;
            }
        });

        ops
    }
}

impl Runnable for VirtualMachine {
    fn run(&mut self, ops: Vec<OpCode>) -> Vec<u8> {
        let mut ptr: usize = 0;
        let mut pc: usize = 0;

        let mut stdout: Vec<u8> = Vec::new();

        while pc < ops.len() {
            let op_code = &ops[pc];

            match op_code {
                OpCode::IncrementPointer => {
                    ptr = ptr.saturating_add(1) % MEM_SIZE;
                }
                OpCode::DecrementPointer => {
                    ptr = ptr.saturating_sub(1) % MEM_SIZE;
                }
                OpCode::Increment => {
                    self.mem[ptr] = self.mem[ptr].wrapping_add(1);
                }
                OpCode::Decrement => {
                    self.mem[ptr] = self.mem[ptr].wrapping_sub(1);
                }
                OpCode::Write => stdout.push(self.mem[ptr]),
                OpCode::Read => {
                    let mut input: [u8; 1] = [0; 1];
                    std::io::stdin()
                        .read_exact(&mut input)
                        .expect("Error: failed to read stdin");
                    self.mem[ptr] = input[0];
                }
                OpCode::LoopBegin => {
                    if self.mem[ptr] == 0 {
                        pc = match self.cache.get(&pc) {
                            Some(val) => *val,
                            None => panic!("Syntax Error: unmatched [ at {}", pc),
                        }
                    }
                }
                OpCode::LoopEnd => {
                    if self.mem[ptr] != 0 {
                        pc = *self.cache.get(&pc).unwrap();
                    }
                }
            }

            pc += 1;
        }

        stdout
    }
}
