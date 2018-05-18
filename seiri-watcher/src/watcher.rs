use notify;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use seiri::config::Config;
use seiri::database::{Connection, ConnectionPool};
use seiri::paths::is_in_hidden_path;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
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

fn is_hidden_file(entry: &PathBuf) -> bool {
    entry
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn list<F>(watch_dir: &str, config: &Config, pool: &ConnectionPool, process: F) -> ()
where
    F: Fn(&Path, &Config, &Connection, bool) -> (),
{
    let watch_dir = Path::new(watch_dir);
    let walker = WalkDir::new(watch_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                process(entry.path(), config, &pool.get().unwrap(), true);
            }
        }
    }
}

pub enum WatchStatus {
    KeepAlive,
    Exit,
}

pub fn watch<F>(
    watch_dir: &str,
    config: Config,
    pool: ConnectionPool,
    process: F,
    quit_rx: Receiver<WatchStatus>,
) -> notify::Result<()>
where
    F: Fn(&Path, &Config, &Connection, bool) -> () + Send + Sync + Copy + 'static,
{
    let (tx, rx) = channel();
    let exec_pool = ThreadPool::new(8);
    let db_pool = Arc::new(pool);
    let config = Arc::new(config);
   // let process = Arc::new(process);
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
        select! {
            event = rx.recv() => match event {
                Ok(event) => {
                    // We only want to process events when the file is idle.
                    // However, if the write finishes before the delay, only the create event is fired.
                    // Otherwise, the write event will be delayed until the latest possible.
                    if let DebouncedEvent::Write(ref path) = event {
                        if check_idle(path) && !is_in_hidden_path(path, watch_dir) && !is_hidden_file(path) {
                            let db_pool = Arc::clone(&db_pool);
                            let config = Arc::clone(&config);
                            let path = path.clone();
                            exec_pool.execute(move || {
                                let pool_ref = &db_pool;
                                let config = config.as_ref();
                                let db_conn = pool_ref.get().unwrap();
                                let path = path.as_path();
                                // Wait two seconds for the track to idle...
                                thread::sleep(Duration::from_secs(2));
                                process(path, config, &db_conn, true);
                            });
                        }
                    }
                    if let DebouncedEvent::Create(ref path) = event {
                        if check_idle(path) && !is_in_hidden_path(path, watch_dir) && !is_hidden_file(path) {
                            let db_pool = db_pool.clone();
                            let config = config.clone();
                            let path = path.clone();
                            exec_pool.execute(move || {
                                let pool_ref = &db_pool;
                                let config = config.as_ref();
                                let db_conn = pool_ref.get().unwrap();
                                let path = path.as_path();
                                // Wait two seconds for the track to idle...
                                thread::sleep(Duration::from_secs(2));
                                process(path, config, &db_conn, true);
                            });
                        }
                    }
                }
                Err(e) => eprintln!("WATCHERROR~{:?}", e),
            },
            keepalive = quit_rx.recv() => match keepalive {
                Ok(WatchStatus::KeepAlive) => (),
                Ok(WatchStatus::Exit) => break,
                Err(_) => break,
            }
        }
    }
    Ok(())
}
