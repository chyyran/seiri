use std::process::Command;
use std::env;
use std::path::PathBuf;
use error::{Error, Result};

extern crate tree_magic;

fn locate_helper() -> Result<PathBuf> {
    if cfg!(target_os = "windows") {
        let mut path = env::current_dir().unwrap();
        path.push("tools");
        path.push("taglibsharp-katatsuki.exe");
        if !path.exists() {
            return Err(Error::HelperNotFound);
        }
        return Ok(path);
    } else {
        let mut path = env::current_dir().unwrap();
        path.push("tools");
        path.push("taglibsharp-katatsuki");
        if !path.exists() {
            return Err(Error::HelperNotFound);
        }
        return Ok(path);
    }
    
    return Err(Error::UnsupportedOS);
}

pub fn call_helper(file_path: &str) -> Result<String> {
    let pathbuf = PathBuf::from(file_path);
    if !pathbuf.exists() {
        return Err(Error::FileNotFound(file_path.to_owned()));
    }
    let mimetype = tree_magic::from_filepath(pathbuf.as_path());
    if !mimetype.starts_with("audio") {
        return Err(Error::UnsupportedFile(pathbuf));
    } 
    let helper = locate_helper();
    match helper {
        Ok(path) => {
            let mut command = Command::new(path);
            command.arg(file_path);
            match command.output() {
                Ok(output) => {
                    if output.status.success() {
                        let result = String::from_utf8(output.stdout).unwrap();
                        Ok(result)
                    } else {
                        Err(Error::UnsupportedFile(pathbuf))
                    }
                }
                Err(_) => Err(Error::HelperNotFound),
            }
        }
        Err(err) => Err(err),
    }
}
