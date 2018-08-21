#[macro_use]
extern crate enum_primitive_derive;

extern crate chrono;
extern crate imagesize;
extern crate libc;
extern crate num_traits;

extern crate libkatatsuki_sys as sys;

use chrono::Local;
use imagesize::blob_size;
use std::ffi::NulError;
use std::ffi::{CStr, CString};
use std::io::{BufReader, Read};
use std::io::{Error, ErrorKind, Result};
use std::os::raw::{c_char, c_void};
use std::path::Path;
use std::slice::from_raw_parts;

pub use num_traits::{FromPrimitive, ToPrimitive};
pub use track::Track;
pub use track::TrackFileType;

mod track;

fn c_str_to_str(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        return None;
    }

    let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };
    let result = if bytes.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(bytes).to_string())
    };

    unsafe {
        sys::free_allocated_data(c_str as *mut c_void);
    }

    result
}

struct TrackData {
    raw: *mut sys::track_data,
}

impl TrackData {
    // Dangerous access here, path not existing is UB.
    pub fn new(path: &CString) -> TrackData {
        TrackData {
            raw: unsafe { sys::create_track_data(path.to_owned().into_raw()) },
        }
    }

    pub fn title(&self) -> String {
        c_str_to_str(unsafe { sys::get_title(self.raw) }).unwrap_or("".to_owned())
    }

    pub fn artist(&self) -> String {
        c_str_to_str(unsafe { sys::get_artist(self.raw) }).unwrap_or("".to_owned())
    }

    pub fn album(&self) -> String {
        c_str_to_str(unsafe { sys::get_album(self.raw) }).unwrap_or("".to_owned())
    }

    pub fn album_artists(&self) -> String {
        c_str_to_str(unsafe { sys::get_album_artist(self.raw) }).unwrap_or("".to_owned())
    }

    pub fn musicbrainz_track_id(&self) -> Option<String> {
        c_str_to_str(unsafe { sys::get_musicbrainz_track_id(self.raw) })
    }

    pub fn year(&self) -> u32 {
        unsafe { sys::get_year(self.raw) }
    }

    pub fn track_number(&self) -> u32 {
        unsafe { sys::get_track_number(self.raw) }
    }

    pub fn bitrate(&self) -> i32 {
        unsafe { sys::get_bitrate(self.raw) }
    }

    pub fn disc_number(&self) -> u32 {
        unsafe { sys::get_disc_number(self.raw) }
    }

    pub fn duration(&self) -> i64 {
        unsafe { sys::get_duration(self.raw) }
    }

    pub fn sample_rate(&self) -> i32 {
        unsafe { sys::get_sample_rate(self.raw) }
    }

    pub fn file_type(&self) -> TrackFileType {
        let file_type = unsafe { sys::get_file_type(self.raw) };
        TrackFileType::from_u32(file_type as u32).unwrap()
    }

    pub fn has_front_cover(&self) -> bool {
        unsafe { sys::has_album_art(self.raw) }
    }

    pub unsafe fn cover_bytes(&self, size: usize) -> CoverBytes {
        CoverBytes {
            raw: sys::get_album_art_all_bytes(self.raw) as *const u8,
        }
    }
}

struct CoverBytes {
    raw: *const u8,
}

impl Drop for CoverBytes {
    fn drop(&mut self) {
        unsafe { sys::free_allocated_data(self.raw as *mut c_void) }
    }
}

impl Drop for TrackData {
    fn drop(&mut self) {
        unsafe { sys::delete_track_data(self.raw) }
    }
}
#[derive(Debug)]
pub enum FileError {
    OpenFailure,
    SaveFailure,
    PathAsString,
    NullPathString(NulError),
    InvalidTagFile,
}

impl Track {
    pub fn from_path(path: &Path, source: Option<&str>) -> Result<Track> {
        if !path.exists() {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("File {:?} not found.", path),
            ))
        } else {
            if let Ok(path_ptr) = path
                .to_owned()
                .to_str()
                .ok_or(FileError::PathAsString)
                .and_then(|path| CString::new(path).map_err(|err| FileError::NullPathString(err)))
            {
                let track: TrackData = TrackData::new(&path_ptr);
                if let TrackFileType::Unknown = track.file_type() {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("File {:?} is unsupported", path),
                    ))
                } else {
                    let mut fcw = 0;
                    let mut fch = 0;
                    if track.has_front_cover() {
                        let bytes = unsafe { track.cover_bytes(384) };
                        let slice = unsafe { from_raw_parts(bytes.raw, 384) };
                        // match blob_size(slice) {
                        //     Ok(size) => {
                        //         fcw = size.width as i32;
                        //         fch = size.height as i32;
                        //     }
                        //     Err(err) => println!("{:?}", err)
                        // }
                        if let Ok(size) = blob_size(slice) {
                            fcw = size.width as i32;
                            fch = size.height as i32;
                            // println!("Width: {}, Height: {}", fcw, fch);
                        }
                        // } else {
                        //     println!("Cover but unreadable");
                        // }
                        // } else {
                        //     println!("No front cover");
                        // }
                    }

                    let track = Ok(Track {
                        file_path: path.to_owned(),
                        file_type: track.file_type(),
                        title: track.title(),
                        artist: track.artist(),
                        album: track.album(),
                        album_artists: track
                            .album_artists()
                            .split(';')
                            .map(|c| c.to_owned())
                            .collect::<Vec<String>>(),
                        year: track.year() as i32,
                        track_number: track.track_number() as i32,
                        musicbrainz_track_id: track.musicbrainz_track_id(),
                        has_front_cover: track.has_front_cover(),
                        front_cover_width: fcw,
                        front_cover_height: fch,
                        bitrate: track.bitrate(),
                        sample_rate: track.sample_rate(),
                        source: source.unwrap_or("None").to_owned(),
                        disc_number: track.disc_number() as i32,
                        duration: track.duration() as i32,
                        updated: Local::now().format("%Y-%m-%d").to_string(),
                    });
                    drop(path_ptr);
                    track
                }
            } else {
                Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("Path was invalid."),
                ))
            }
        }
    }
}
