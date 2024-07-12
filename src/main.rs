use std::{fs, process::exit, time::Instant};

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

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error: cannot read file");
            exit(1);
        }
    };

    let mut interpreter = VirtualMachine::new();

    let start = Instant::now();
    interpreter.exec(content.clone().into());
    let duration = start.elapsed();

    println!("\n=== Code Execution Succesful {:?} ===", duration);
}
