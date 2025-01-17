#[allow(unused_imports)]
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input == "exit 0" {
            return ExitCode::from(0);
        }

        println!("{}: command not found", input);
    }
}
