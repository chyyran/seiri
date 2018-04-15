extern crate notify;

use std::path::PathBuf;
use std::fs::OpenOptions;
use std::fs;
use std::io;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
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

pub fn list<F>(watch_dir: &str, process: F) -> ()
where
    F: Fn(&PathBuf) -> (),
{
    if let Ok(paths) = fs::read_dir(watch_dir) {
        for result in paths {
            if let Ok(path) = result {
                process(&path.path());
            }
        }
    }
}

pub fn watch<F>(watch_dir: &str, process: F) -> notify::Result<()>
where
    F: Fn(&PathBuf) -> (),
{
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_dir, RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.

    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Write(ref path) = event {
                    if check_idle(path) {
                        process(path);
                    }
                }
                if let DebouncedEvent::Create(ref path) = event {
                    if check_idle(path) {
                        process(path);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
