use std::process::{Command, Output};
use std::env;
use std::io;
use std::path::PathBuf;

fn locate_helper() -> Result<PathBuf, io::Error> {
    if cfg!(target_os = "windows") {
        let mut path = env::current_dir().unwrap();
        path.push("tools");
        path.push("win-x64");
        path.push("taglibsharp-katatsuki");
        path.push("taglibsharp-katatsuki.exe");
        if !path.exists() {
            let err = io::Error::new(io::ErrorKind::NotFound, "Could not find TagLibSharp Helper");
            return Err(err);
        }
        return Ok(path)
    }
    let err = io::Error::new(io::ErrorKind::NotFound, "Unsupported Operating System");
    return Err(err);
}

pub fn call_helper(file_path: &str) -> Option<String> {
    let helper = locate_helper();
    if let Ok(path) = helper {
        let mut command = Command::new(path);
        command.arg(file_path);
        let result = command.output().ok().and_then(|output: Output| {
            if output.status.success() {
                Some(String::from_utf8(output.stdout).unwrap())
            } else {
                None
            }
        });
        result
    } else {
        None
    }
}
