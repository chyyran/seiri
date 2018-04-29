#[macro_use]
extern crate enum_primitive_derive;

extern crate libc;
extern crate widestring;
extern crate chrono;
extern crate num_traits;

extern crate libkatatsuki_sys as sys;

mod track;
const TICKS_PER_MS: i64 = 10000;
const NS_PER_TICK: i64 = 100;
const SEC_PER_MS: i64 = 1000;

fn ticks_to_ms(ticks: i64) -> i32 {
    (ticks / TICKS_PER_MS) as i32
}

use std::path::Path;
use std::io::{Error, ErrorKind, Result};

use sys::katatsuki_Track;
use sys::katatsuki_get_track_data;

pub use track::Track;
pub use track::TrackFileType;
pub use num_traits::{FromPrimitive, ToPrimitive};

impl Track {
    pub fn from_path(path: &Path) -> Result<Track> {
       if !path.exists() {
           Err(Error::new(ErrorKind::NotFound, format!("File {:?} not found.", path)))
       } else {
           
       }
    }
} 


