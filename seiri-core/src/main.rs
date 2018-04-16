#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate lazy_static;
extern crate notify;
extern crate serde_json;
extern crate tree_magic;
extern crate rusqlite;
extern crate regex;

use std::path::PathBuf;
use std::thread;

use error::Error;

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
            Error::MissingRequiredTag(file_name, tag) => println!("Found track {} but missing tag {}", file_name, tag),
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

    utils::wait_for_exit();
}
