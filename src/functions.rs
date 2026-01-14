use std::{
    collections::HashSet, 
    process::ExitCode,
};

pub fn echo(input: Vec<&str>) -> () {
    println!("{}", input.join(" "));
}

pub fn command_type(input: Vec<&str>) -> () {
    let hs = HashSet::from(["echo", "type", "exit"]);
    let cmd = input.first().unwrap();

    if hs.contains(cmd) {
        println!("{} is a shell builtin", cmd);
    } else {
        println!("{}: not found", cmd);
    }
}

pub fn exit_shell(input: &str) -> ExitCode {
    let status = input.parse::<u8>().unwrap_or(0);
    ExitCode::from(status)
}