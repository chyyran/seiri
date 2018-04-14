extern crate notify;

use std::path::PathBuf;
use std::fs::OpenOptions;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use notify::DebouncedEvent;

fn check_idle(path: &PathBuf) -> bool {
    return match OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .truncate(false)
        .open(&path)
    {
        Err(_) => false,
        Ok(_) => true,
    };
}

pub fn watch(watch_dir: &str) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch(watch_dir, RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.

    loop {
        match rx.recv() {
            Ok(event) => match &event {
                &DebouncedEvent::Write(ref path) => {
                    if check_idle(path) {
                        println!("File settled at {:?} from write event", path);
                    }
                }
                &DebouncedEvent::Create(ref path) => {
                    if check_idle(path) {
                        println!("File settled at {:?} from create event.", path);
                    }
                }
                _ => println!("Other event occurred: {:?}", event),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
