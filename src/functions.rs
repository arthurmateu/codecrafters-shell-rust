use std::{
    collections::HashSet, env, io::{self, Write}, os::unix::fs::PermissionsExt, path::{Path, PathBuf}, process::{Command, ExitCode, Output}
};

pub fn echo(input: Vec<&str>) -> () {
    println!("{}", input.join(" "));
}

pub fn command_type(input: Vec<&str>) -> () {
    let builtins = HashSet::from(["echo", "type", "exit"]);
    let cmd = input.first().unwrap();
    let full_path = which(cmd);

    if builtins.contains(cmd) {
        println!("{} is a shell builtin", cmd);
    } else if full_path.is_some() {
        println!("{} is {}", cmd, full_path.unwrap());
    } else {
        println!("{}: not found", cmd);
    }
}

pub fn run_command(cmd: &str, args: Vec<&str>) -> () {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute process");

    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);
}

pub fn which(cmd: &str) -> Option<String> {
    env::var("PATH")
        .unwrap()
        .split(":")
        .map(Path::new)
        .find_map(|dir| {
            let full = dir.join(cmd);
            if full.is_file() && is_executable(&full) {
                full.into_os_string().into_string().ok()
            } else {
                None
            }
        })
}

fn is_executable(cmd: &PathBuf) -> bool {
    cmd.metadata().unwrap().permissions().mode() & 0o111 != 0
}

pub fn exit_shell(input: &str) -> ExitCode {
    let status = input.parse::<u8>().unwrap_or(0);
    ExitCode::from(status)
}