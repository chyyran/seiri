#[macro_use]
extern crate enum_primitive_derive;

extern crate chrono;
extern crate libc;
extern crate num_traits;
extern crate imagesize;

extern crate libkatatsuki_sys as sys;

use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::ffi::{CStr, CString};
use libc::c_char;
use sys::katatsuki_Track;
use sys::katatsuki_get_track_data;
use std::slice::from_raw_parts;
use imagesize::blob_size;

use chrono::Local;

pub use num_traits::{FromPrimitive, ToPrimitive};
pub use track::Track;
pub use track::TrackFileType;

mod track;
const TICKS_PER_MS: i64 = 10000;

fn ticks_to_ms(ticks: i64) -> i32 {
    (ticks / TICKS_PER_MS) as i32
}

fn c_str_to_str(c_str: *const c_char) -> Option<String> {
    if c_str.is_null() {
        return None;
    }

    let bytes = unsafe { CStr::from_ptr(c_str).to_bytes() };
    if bytes.is_empty() {
        None
    } else {
        Some(String::from_utf8_lossy(bytes).to_string())
    }
}

impl Track {
    pub fn from_path(path: &Path, source: Option<&str>) -> Result<Track> {

        if !path.exists() {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("File {:?} not found.", path),
            ))
        } else {
            if let Ok(path_ptr) = CString::new(path.as_os_str().to_string_lossy().as_ref()) {
                let track: katatsuki_Track = unsafe { katatsuki_get_track_data(path_ptr.into_raw()) };
                if track.FileType == 0 {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("File {:?} is unsupported", path),
                    ))
                } else {
                    let mut fcw = 0;
                    let mut fch = 0;
                    if !track.CoverBytes.is_null() {
                        let slice = unsafe { from_raw_parts(track.CoverBytes, 32) };
                        if let Ok(size) = blob_size(slice) {
                            fcw = size.width as i32;
                            fch = size.height as i32;
                        }
                    }
                    
                    Ok(Track {
                        file_path: path.to_owned(),
                        file_type: TrackFileType::from_u32(track.FileType).unwrap(),
                        title: c_str_to_str(track.Title).unwrap_or("".to_owned()),
                        artist: c_str_to_str(track.Artist).unwrap_or("".to_owned()),
                        album: c_str_to_str(track.Album).unwrap_or("".to_owned()),
                        album_artists: c_str_to_str(track.AlbumArtists)
                            .unwrap_or("".to_owned())
                            .split(';')
                            .map(|c| c.to_owned())
                            .collect::<Vec<String>>(),
                        year: track.Year as i32,
                        track_number: track.TrackNumber as i32,
                        musicbrainz_track_id: c_str_to_str(track.MusicBrainzTrackId),
                        has_front_cover: track.HasFrontCover,
                        front_cover_width: fcw,
                        front_cover_height: fch,
                        bitrate: track.Bitrate,
                        sample_rate: track.SampleRate,
                        source: source.unwrap_or("None").to_owned(),
                        disc_number: track.DiscNumber as i32,
                        duration: ticks_to_ms(track.Duration),
                        updated: Local::now().format("%Y-%m-%d").to_string(),
                    })
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
