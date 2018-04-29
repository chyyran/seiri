#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

use libc::{c_int, c_uint, c_long};

#[derive(Debug)]
#[repr(C)]
pub struct katatsuki_Track {
    FileType:  c_uint,
    Title: *const u16,
    Artist: *const u16,
    AlbumArtists: *const u16,
    Album: *const u16,
    Year: c_uint,
    TrackNumber: c_uint,
    MusicBrainzTrackId: *const u16,
    HasFrontCover: bool,
    FrontCoverHeight: c_int,
    FrontCoverWidth: c_int,
    Bitrate: c_int,
    SampleRate: c_int,
    DiscNumber: c_uint,
    Duration: c_long,
}

#[link(name = "libkatatsuki", kind = "static")]
#[link(name = "bootstrapperdll", kind = "static")]
#[link(name = "Runtime",  kind = "static")]
extern "C" {
    pub fn katatsuki_get_track_data(file_path: *const u16) -> katatsuki_Track;
}

// pub fn main() {
//     let track_path = Path::new("track.flac");
//     let track_path_ptr = WideCString::from_str(track_path.as_os_str()).unwrap();
//     unsafe {
//         let track = katatsuki_get_track_data(track_path_ptr.as_ptr());
//         println!("{:?}", track);
//     }
// }