#![feature(fs_read_write)]
#![feature(toowned_clone_into)]

#[macro_use]
extern crate juniper;

#[macro_use]
extern crate quick_error;

extern crate chrono;
extern crate humantime;
extern crate itertools;

extern crate rand;
extern crate regex;
extern crate rusqlite;

mod bangs;
mod error;
mod track;

pub use track::TrackFileType;
pub use track::Track;
pub use error::{Error, Result};
pub use bangs::Bang;

pub mod database;

pub mod ticks {
    pub use bangs::ms_to_ticks;
    pub use bangs::ticks_to_ms;
}