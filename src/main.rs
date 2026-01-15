#[allow(unused_imports)]
mod functions;
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

        let mut args: Vec<&str> = input.split_whitespace().collect();
        let command: &str = if args.len() > 0 { args[0] } else { " " };
        args = if args.len() > 1 { args[1..].to_vec() } else { vec![" "] };

        match command {
            "echo" => functions::echo(args),
            "type" => functions::command_type(args),
            "exit" => return functions::exit_shell(args[0]),
            " " => continue,
            _ => {
                let cmd = functions::which(command);
                if cmd.is_some() {
                    functions::run_command(command, args);
                } else {
                    println!("{}: command not found", command)
                }
            }
        }
    }
}
