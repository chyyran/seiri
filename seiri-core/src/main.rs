extern crate notify;
extern crate taglib;

use std::path::PathBuf;
use std::thread;

mod utils;
mod watcher;
mod track;

fn process(path : &PathBuf) {
    println!("Found path to be watched {:?}", path);
}

fn main() {
    thread::spawn(move || {
        watcher::list("C:\\watch", process);
        if let Err(e) = watcher::watch("C:\\watch", process) {
            println!("{}", e);
        }
    });

    let track_title = track::Track::get_track_title("C:\\watch\\1-07 Alone.flac");
    
    if let Ok(title) = track_title {
        if let Some(title) = title {
            println!("{}", title);
        }
    }

    utils::wait_for_exit();
}
