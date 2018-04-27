#![feature(fs_read_write)]
#![feature(toowned_clone_into)]
#![feature(ascii_ctype)]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate quick_error;

extern crate chrono;
extern crate humantime;
extern crate itertools;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate app_dirs;

mod bangs;
mod error;
mod track;
mod taglibsharp;

pub use track::TrackFileType;
pub use track::Track;
pub use track::TaglibTrack;
pub use error::{Error, Result};
pub use bangs::Bang;

pub mod database;
pub mod paths;

pub mod ticks {
    pub use bangs::ms_to_ticks;
    pub use bangs::ticks_to_ms;
}