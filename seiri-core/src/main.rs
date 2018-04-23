#![feature(fs_read_write)]
#![feature(toowned_clone_into)]
#![feature(mpsc_select)]

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

use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

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
use watcher::WatchStatus;

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
            }
            Error::MissingRequiredTag(file_name, tag) => {
                println!("Found track {} but missing tag {}", file_name, tag)
            }
            Error::HelperNotFound => println!("Katatsuki TagLib helper not found."),
            _ => {}
        },
    }
}

fn wait_for_watch_root_available(folder: &str) -> (PathBuf, PathBuf) {
    println!("Waiting for folder {}...", folder);
    let wait_time = Duration::from_secs(5);
    while let Err(_) = paths::ensure_music_folder(folder) {
        thread::park_timeout(wait_time);
    }
    println!("Successfully ensured folder {}", folder);
    paths::ensure_music_folder(folder).unwrap()
}

fn begin_watch(config: &Config, rx: Receiver<WatchStatus>) {
    let auto_paths = wait_for_watch_root_available(&config.music_folder);
    let watch_path = &auto_paths.1.to_str().unwrap();
    println!("Watching {}", watch_path);
    watcher::list(&watch_path, &config, process);
    // Create a channel to receive the events.
    if let Err(e) = watcher::watch(&watch_path, &config, process, rx) {
        println!("{}", e);
    }
}

fn get_watcher_thread(rx: Receiver<WatchStatus>) -> io::Result<thread::JoinHandle<()>> {
    thread::Builder::new()
        .name("WatchThread".to_string())
        .spawn(move || {
            let config = config::get_config();
            begin_watch(&config, rx)
        })
}

fn start_watcher_watchdog(wait_time: Duration) {
    thread::spawn(move || {
        let (tx, rx) = channel();
        let mut tx = tx;
        let config = config::get_config();
        wait_for_watch_root_available(&config.music_folder);
        let mut _watch_thread = get_watcher_thread(rx).unwrap();
        loop {
            thread::park_timeout(wait_time);
            if let Err(_) = tx.send(WatchStatus::KeepAlive) {
                println!("Keep-alive failed. Watcher thread probably panicked. Restarting Watcher Thread...");
                let (new_tx, rx) = channel();
                tx = new_tx.clone();
                _watch_thread = get_watcher_thread(rx).unwrap();
            }

            let music_folder = paths::ensure_music_folder(&config.music_folder);
            if let Err(_) = music_folder {
                println!("Lost access to {}", &config.music_folder);
                wait_for_watch_root_available(&config.music_folder);
                let (new_tx, rx) = channel();
                tx.send(WatchStatus::Exit).unwrap();
                println!("Requested watcher thread exit. Restarting Watcher Thread...");
                tx = new_tx.clone();
                _watch_thread = get_watcher_thread(rx).unwrap();
            }
        }
    });
}
fn main() {
    let wait_time = Duration::from_secs(5);
    start_watcher_watchdog(wait_time);
    let conn = paths::get_database_connection();
    utils::wait_for_exit(&conn);
}
