#![feature(fs_read_write)]
#![feature(toowned_clone_into)]

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate serde_derive;

extern crate app_dirs;
extern crate humantime;
extern crate itertools;
extern crate notify;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate serde_json;
extern crate tree_magic;
extern crate toml;

use std::path::PathBuf;
use std::thread;
use std::env;

use rusqlite::Connection;
use rusqlite::OpenFlags;


mod utils;
mod watcher;
mod track;
mod taglibsharp;
mod error;
mod database;
mod bangs;
mod paths;
mod config;

use error::Error;

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
        let config = paths::get_config();
        paths::ensure_music_folder(&config.music_folder);
        watcher::list(&config.music_folder, process);
        if let Err(e) = watcher::watch(&config.music_folder, process) {
            println!("{}", e);
        }
    });
    let appdata_path = paths::get_appdata_path();
    let config = paths::get_config();
    println!("{:?}", config);
    println!("{:?}", appdata_path);
    let conn = paths::get_database_connection();
    utils::wait_for_exit(&conn);
}
