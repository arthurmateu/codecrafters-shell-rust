#[allow(unused_imports)]
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn echo(input: Vec<&str>) -> () {
    for w in input {
        print!("{w} ");
    }
    println!();
}

fn command_type(input: Vec<&str>) -> () {
    let valid_commands = vec!["echo", "type", "exit"];

    for cmd in input {
        if valid_commands.contains(&cmd) {
            println!("{} is a shell builtin", cmd);
        } else {
            println!("{}: not found", cmd);
        }
    }
}

fn exit_shell(input: &str) -> ExitCode {
    let status = input.parse::<u8>().unwrap_or(0);
    ExitCode::from(status)
}

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();

        let mut command_args: Vec<&str> = input.split_whitespace().collect();
        let mut command: &str = " ";

        if command_args.len() > 0 {
            command = command_args[0];
        }
        if command_args.len() > 1 {
            command_args = command_args[1..].to_vec();
        }

        if command == "echo" {
            echo(command_args);
        } else if command == "type" {
            command_type(command_args);
        } else if command == "exit" {
            return exit_shell(command_args[0]);
        } else if command == " " {
            // do nothing
        } else {
            println!("{}: command not found", command);
        }
    }
}
