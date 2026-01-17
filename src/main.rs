#[allow(unused_imports)]
mod functions;
use std::{
    io::{self, Write},
    process::ExitCode,
};
use shlex;

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input: String = String::new();
        stdin.read_line(&mut input).unwrap();
        let input  = shlex::split(input.trim()).unwrap();
        
        let command = input.first().map_or(" ", |v| v);
        let args: Option<Vec<&str>> = if input.len() > 1 {
            Some(input[1..].iter().map(|c| c.as_str()).collect())
        } else {
            None
        };
        
        match command {
            "echo" => functions::echo(args),
            "type" => functions::command_type(args),
            "pwd" => functions::pwd(),
            "cd" => functions::change_directory(args),
            "exit" => return functions::exit_shell(args),
            "None" => continue,
            _ => {
                if functions::which(command).is_some() {
                    functions::run_command(command, args);
                } else {
                    println!("{}: command not found", command)
                }
            }
        }
    }
}
