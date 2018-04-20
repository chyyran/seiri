#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate lazy_static;

extern crate rand;
extern crate humantime;
extern crate itertools;
extern crate notify;
extern crate regex;
extern crate rusqlite;
extern crate serde_json;
extern crate tree_magic;

use std::path::{PathBuf, Path};
use std::thread;
use std::env;

use error::Error;
use rusqlite::Connection;
use rusqlite::OpenFlags;
mod utils;
mod watcher;
mod track;
mod taglibsharp;
mod error;
mod database;
mod bangs;

fn process(path: &PathBuf) {
    let track = track::Track::new(path, None);
    match track {
        Ok(tagdata) => println!(
            "Found track {} - {} - {}",
            tagdata.title, tagdata.artist, tagdata.file_type
        ),
        Err(err) => match err {
            Error::UnsupportedFile(file_name) => println!("Found non-track item {}", file_name),
            Error::MissingRequiredTag(file_name, tag) => {
                println!("Found track {} but missing tag {}", file_name, tag)
            }
            Error::HelperNotFound => println!("Katatsuki TagLib helper not found."),
            _ => {}
        },
    }
}

fn main() {
    thread::spawn(move || {
        watcher::list("C:\\watch", process);
        if let Err(e) = watcher::watch("C:\\watch", process) {
            println!("{}", e);
        }
    });
    let mut path = env::current_dir().unwrap();
    path.push("tracks.db");
    println!("{:?}", path);
    let conn = Connection::open_with_flags(path.as_path(), OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    utils::wait_for_exit(&conn);
}
