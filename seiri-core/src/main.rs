extern crate notify;
extern crate taglib;

use std::path::PathBuf;
use std::thread;
use std::fs::File;
use std::io::Write;

mod utils;
mod watcher;
mod track;
mod taglibsharp;

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

    let track_data = taglibsharp::call_helper("C:\\watch\\1-08 seeds.flac");
    let file = File::create("foo.txt");
    let track_data = track_data.unwrap();
    file.unwrap().write_all(&track_data.as_bytes());
    println!("{}", &track_data);
    utils::wait_for_exit();
}
