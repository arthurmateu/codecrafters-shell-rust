#[allow(unused_imports)]
use std::io::{self, Write};

pub fn run_command(command: String) -> String {
    format!("{command}: command not found")
}

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    println!("{}", run_command(input.trim().to_string()));
}
