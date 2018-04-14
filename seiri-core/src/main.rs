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

    utils::wait_for_exit();
}
