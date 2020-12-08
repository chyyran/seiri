extern crate quick_error;
extern crate chrono;
extern crate humantime;
extern crate itertools;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate app_dirs;
extern crate toml;
extern crate katatsuki;
extern crate dirs;

mod bangs;
mod error;


pub use katatsuki::TrackFileType;
pub use katatsuki::Track;
pub use self::error::{Error, Result, ConfigErrorType};
pub use self::bangs::Bang;

pub mod config;
pub mod database;
pub mod paths;

pub mod ticks {
    pub use crate::bangs::ms_to_ticks;
    pub use crate::bangs::ticks_to_ms;
}
