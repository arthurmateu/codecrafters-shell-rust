use std::{
    collections::HashSet, env, io::{self, Write}, os::unix::fs::PermissionsExt, path::{Path, PathBuf}, process::{Command, ExitCode, Output}
};

pub fn echo(input: Option<Vec<&str>>) -> () {
    println!("{}", input.unwrap_or_default().join(" "));
}

pub fn command_type(args: Option<Vec<&str>>) -> () {
    let builtins = HashSet::from(["echo", "type", "exit", "pwd", "cd"]);
    if args.is_none() {
        println!("Usage: type <command>");
        return
    }
    let tmp = args.unwrap();
    let cmd = tmp.first().unwrap();

    if builtins.contains(cmd) {
        println!("{cmd} is a shell builtin");
    } else if let Some(path) = which(cmd) {
        println!("{cmd} is {path}");
    } else {
        println!("{cmd}: not found");
    }
}

pub fn run_command(cmd: &str, args: Option<Vec<&str>>) -> () {
    let output = Command::new(cmd)
        .args(args.unwrap_or_default())
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

pub fn change_directory(args: Option<Vec<&str>>) -> () {
    let path = args.unwrap_or(vec!["~"])
                           .first()
                           .expect("No directory given")
                           .replace("~", env::home_dir().unwrap().to_str().unwrap());

    if env::set_current_dir(&path).is_err() {
        println!("cd: {}: No such file or directory", path)
    }
}

fn is_executable(cmd: &PathBuf) -> bool {
    cmd.metadata().unwrap().permissions().mode() & 0o111 != 0
}

pub fn pwd() -> () {
    println!("{}", working_directory());
}

fn working_directory() -> String {
    env::current_dir().expect("Insufficient permissions to access current directory").to_str().unwrap().to_string()
}

pub fn exit_shell(input: Option<Vec<&str>>) -> ExitCode {
    let tmp = input.unwrap_or(vec![&"0"]);
    let status = tmp.first()
                        .unwrap()
                        .parse::<u8>()
                        .unwrap_or(0);

    ExitCode::from(status)
}