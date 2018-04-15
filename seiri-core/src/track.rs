extern crate taglib;

use taglib::*;
use std::time::Duration;

enum TrackFileType {
    Flac,
    Flac4,
    Flac8,
    Flac16,
    Flac24,
    Flac32,
    Alac,
    Mp3Cbr,
    Mp3Vbr,
    Aac,
    Vorbis,
    Opus,
    Wavpack,
    Ape,
    Unknown,
}

pub struct Track {
    file_path: String,
    title: String,
    artist: String,
    album_artists: Vec<String>,
    album: String,
    year: u32,
    track_number: u32,
    musicbrainz_track_id: String,
    has_front_cover: bool,
    front_cover_height: u32,
    front_cover_width: u32,
    bitrate: u32,
    sample_rate: u32,
    source: String,
    duration: Duration,
    file_type: TrackFileType
}

impl Track {
    pub fn get_track_title(file_path: &str) -> Result<Option<String>, taglib::FileError> {
        let track = File::new(file_path);
        Ok(track?.tag()?.title)
    }
}