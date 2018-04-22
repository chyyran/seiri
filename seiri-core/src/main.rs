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
extern crate toml;
extern crate tree_magic;

use std::path::PathBuf;
use std::thread;

mod bangs;
mod config;
mod database;
mod error;
mod paths;
mod taglibsharp;
mod track;
mod utils;
mod watcher;

use error::Error;

fn process(path: &PathBuf) {
    let track = track::Track::new(path, None);
    match track {
        Ok(track) => {
            let config = paths::get_config();
            let library_path = paths::ensure_music_folder(&config.music_folder).0;
            let tagdata = paths::move_track(&track, &library_path);
        }
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
        let auto_paths = paths::ensure_music_folder(&config.music_folder);
        let watch_path = &auto_paths.1.to_str().unwrap();
        println!("Watching {}", watch_path);
        watcher::list(watch_path, process);
        if let Err(e) = watcher::watch(watch_path, process) {
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
