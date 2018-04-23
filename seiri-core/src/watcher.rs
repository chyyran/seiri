extern crate notify;

use config::Config;

use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use paths::is_in_hidden_path;
use std::fs;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use walkdir::{DirEntry, WalkDir};

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

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn list<F>(watch_dir: &str, config: &Config, process: F) -> ()
where
    F: Fn(&Path, &Config) -> (),
{
    let watch_dir = Path::new(watch_dir);
    let walker = WalkDir::new(watch_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
             if entry.file_type().is_file() {
                 process(entry.path(), config);
             }
        }
    }
}

pub fn watch<F>(watch_dir: &str, config: &Config, process: F) -> notify::Result<()>
where
    F: Fn(&Path, &Config) -> (),
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

    let watch_dir = Path::new(watch_dir);
    loop {
        match rx.recv() {
            Ok(event) => {
                // We only want to process events when the file is idle.
                // However, if the write finishes before the delay, only the create event is fired.
                // Otherwise, the write event will be delayed until the latest possible.
                if let DebouncedEvent::Write(ref path) = event {
                    if check_idle(path) && !is_in_hidden_path(path, watch_dir) {
                        process(path, config);
                    }
                }
                if let DebouncedEvent::Create(ref path) = event {
                    if check_idle(path) && !is_in_hidden_path(path, watch_dir) {
                        process(path, config);
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
