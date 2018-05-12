#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use libc::{c_char, c_int, c_longlong, c_uint};
use std::ffi::CString;

#[derive(Debug)]
#[repr(C)]
pub struct katatsuki_Track {
    pub FileType: c_uint,
    pub Title: *const c_char,
    pub Artist: *const c_char,
    pub AlbumArtists: *const c_char,
    pub Album: *const c_char,
    pub Year: c_uint,
    pub TrackNumber: c_uint,
    pub MusicBrainzTrackId: *const c_char,
    pub HasFrontCover: bool,
    pub FrontCoverHeight: c_int,
    pub FrontCoverWidth: c_int,
    pub Bitrate: c_int,
    pub SampleRate: c_int,
    pub DiscNumber: c_uint,
    pub Duration: c_longlong,
}

// #[link(name = "libkatatsuki", kind = "static")]
#[link(name = "bootstrapperdll", kind = "static")]
#[link(name = "Runtime", kind = "static")]
extern "C" {
    pub fn katatsuki_get_track_data(file_path: *const c_char) -> katatsuki_Track;
    pub fn free_corert(ptr: *const c_char) -> ();
}


impl Drop for katatsuki_Track {
    fn drop(&mut self) {
        unsafe {
            free_corert(self.Title);
            free_corert(self.Artist);
            free_corert(self.Album);
            free_corert(self.AlbumArtists);
            free_corert(self.MusicBrainzTrackId);
        }
    }
}
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
