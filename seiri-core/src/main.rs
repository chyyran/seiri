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
extern crate walkdir;

use std::path::{PathBuf, Path};
use std::thread;
use std::time::{Duration};

mod bangs;
mod config;
mod database;
mod error;
mod paths;
mod taglibsharp;
mod track;
mod utils;
mod watcher;

use config::Config;
use error::Error;

fn process(path: &Path, config: &Config) {
    let track = track::Track::new(path, None);
    match track {
        Ok(track) => match paths::ensure_music_folder(&config.music_folder) {
            Ok(library_path) => {
                let track = paths::move_track(&track, &library_path.0, &library_path.1);
                println!("{:?}", track);
            }
            Err(err) => println!("Error {} ocurred when attempting to move track.", err),
        },
        Err(err) => match err {
            Error::UnsupportedFile(file_name) => {
                match paths::ensure_music_folder(&config.music_folder) {
                    Ok(library_path) => {
                        paths::move_non_track(&file_name, &library_path.1).unwrap();
                        println!("Found and moved non-track item {:?}", file_name)
                    }
                    Err(err) => println!("Error {} ocurred when attempting to move track.", err),
                };
            },
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
        let config = config::get_config();
        println!("Waiting for folder {}...", &config.music_folder);
        let wait_time = Duration::from_secs(5);
        while let Err(_) = paths::ensure_music_folder(&config.music_folder) {
            thread::park_timeout(wait_time);
        }
        println!("Successfully ensured folder {}", &config.music_folder);
        let auto_paths = paths::ensure_music_folder(&config.music_folder).unwrap();
        let watch_path = &auto_paths.1.to_str().unwrap();
        println!("Watching {}", watch_path);
        watcher::list(&watch_path, &config, process);
        if let Err(e) = watcher::watch(&watch_path, &config, process) {
            println!("{}", e);
        }
    });
    let appdata_path = paths::get_appdata_path();
    println!("{:?}", appdata_path);
    let conn = paths::get_database_connection();
    utils::wait_for_exit(&conn);
}
