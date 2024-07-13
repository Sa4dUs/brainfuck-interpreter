use std::{fs::File, io::Read, process::exit, time::Instant};

use clap::Parser;
use rustfuck::interpreter::*;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;

    if !path.is_file() {
        eprintln!("Error: the specified path is not a file");
        exit(1)
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: cannot open file");
            exit(1);
        }
    };
    
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: cannot read target file");
            exit(1);
        }
    }

    let mut interpreter = VirtualMachine::new();

    let start = Instant::now();
    interpreter.exec(buffer);
    let duration = start.elapsed();

    println!("\n=== Code Execution Succesful {:?} ===", duration);
}
