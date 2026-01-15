use std::{
    collections::HashSet, env, os::unix::fs::PermissionsExt, path::{Path, PathBuf}, process::ExitCode
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

fn which(cmd: &str) -> Option<String> {
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