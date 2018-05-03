#[macro_use]
extern crate enum_primitive_derive;

extern crate chrono;
extern crate libc;
extern crate num_traits;
extern crate widestring;

extern crate libkatatsuki_sys as sys;

use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use sys::katatsuki_Track;
use sys::katatsuki_get_track_data;

use chrono::Local;
use widestring::WideCString;

pub use num_traits::{FromPrimitive, ToPrimitive};
pub use track::Track;
pub use track::TrackFileType;

mod track;
const TICKS_PER_MS: i64 = 10000;

fn ticks_to_ms(ticks: i64) -> i32 {
    (ticks / TICKS_PER_MS) as i32
}

fn wide_string_ptr_to_string(pointer: *const u16) -> Option<String> {
    unsafe {
        if !pointer.is_null() {
            Some(WideCString::from_ptr_str(pointer).to_string_lossy())
        } else {
            None
        }
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
            if let Ok(path_ptr) = WideCString::from_str(path.as_os_str()) {
                let track: katatsuki_Track = unsafe { katatsuki_get_track_data(path_ptr.as_ptr()) };
                if track.FileType == 0 {
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("File {:?} is unsupported", path),
                    ))
                } else {
                    Ok(Track {
                        file_path: path.to_owned(),
                        file_type: TrackFileType::from_u32(track.FileType).unwrap(),
                        title: wide_string_ptr_to_string(track.Title).unwrap_or("".to_owned()),
                        artist: wide_string_ptr_to_string(track.Artist).unwrap_or("".to_owned()),
                        album: wide_string_ptr_to_string(track.Album).unwrap_or("".to_owned()),
                        album_artists: wide_string_ptr_to_string(track.AlbumArtists).unwrap_or("".to_owned())
                            .split(';')
                            .map(|c| c.to_owned())
                            .collect::<Vec<String>>(),
                        year: track.Year as i32,
                        track_number: track.TrackNumber as i32,
                        musicbrainz_track_id: wide_string_ptr_to_string(track.MusicBrainzTrackId),
                        has_front_cover: track.HasFrontCover,
                        front_cover_width: track.FrontCoverWidth,
                        front_cover_height: track.FrontCoverHeight,
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
