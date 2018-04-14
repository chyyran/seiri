extern crate notify;

mod utils;
mod watcher;


fn main() {
    
    watcher::watch("C:\\watch");
    utils::pause();
}