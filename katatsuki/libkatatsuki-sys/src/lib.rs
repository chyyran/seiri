#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use libc::{c_int, c_uint, c_longlong};

#[derive(Debug)]
#[repr(C)]
pub struct katatsuki_Track {
    pub FileType:  c_uint,
    pub Title: *const u16,
    pub Artist: *const u16,
    pub AlbumArtists: *const u16,
    pub Album: *const u16,
    pub Year: c_uint,
    pub TrackNumber: c_uint,
    pub MusicBrainzTrackId: *const u16,
    pub HasFrontCover: bool,
    pub FrontCoverHeight: c_int,
    pub FrontCoverWidth: c_int,
    pub Bitrate: c_int,
    pub SampleRate: c_int,
    pub DiscNumber: c_uint,
    pub Duration: c_longlong,
}

// #[link(name = "libkatatsuki", kind = "static")]
// #[link(name = "bootstrapperdll", kind = "static")]
// #[link(name = "Runtime",  kind = "static")]
extern "C" {
    pub fn katatsuki_get_track_data(file_path: *const u16) -> katatsuki_Track;
}

// extern crate widestring;
// pub fn main() {
//     use std::path::Path;
//     use widestring::WideCString;
//     let track_path = Path::new("track.flac");
//     let track_path_ptr = WideCString::from_str(track_path.as_os_str()).unwrap();
//     unsafe {
//         let track = katatsuki_get_track_data(track_path_ptr.as_ptr());
//         println!("{:?}", track);
//     }
// }