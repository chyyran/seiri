use error::{Error, Result};
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum TrackFileType {
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
    MonkeysAudio,
    Unknown,
}

impl TrackFileType {
    pub fn value(&self) -> i32 {
        match *self {
            TrackFileType::Flac => 0,
            TrackFileType::Flac4 => 1,
            TrackFileType::Flac8 => 2,
            TrackFileType::Flac16 => 3,
            TrackFileType::Flac24 => 4,
            TrackFileType::Flac32 => 5,
            TrackFileType::Alac => 6,
            TrackFileType::Mp3Cbr => 7,
            TrackFileType::Mp3Vbr => 8,
            TrackFileType::Aac => 9,
            TrackFileType::Vorbis => 10,
            TrackFileType::Opus => 11,
            TrackFileType::Wavpack => 12,
            TrackFileType::MonkeysAudio => 13,
            TrackFileType::Unknown => 14,
        }
    }
}

impl From<i32> for TrackFileType {
    fn from(i: i32) -> Self {
        match i {
            0 => TrackFileType::Flac,
            1 => TrackFileType::Flac4,
            2 => TrackFileType::Flac8,
            3 => TrackFileType::Flac16,
            4 => TrackFileType::Flac24,
            5 => TrackFileType::Flac32,
            6 => TrackFileType::Alac,
            7 => TrackFileType::Mp3Cbr,
            8 => TrackFileType::Mp3Vbr,
            9 => TrackFileType::Aac,
            10 => TrackFileType::Vorbis,
            11 => TrackFileType::Opus,
            12 => TrackFileType::Wavpack,
            13 => TrackFileType::MonkeysAudio,
            _ => TrackFileType::Unknown,
        }
    }
}

impl fmt::Display for TrackFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_name = match *self {
            TrackFileType::Flac => "flac",
            TrackFileType::Flac4 => "flac4",
            TrackFileType::Flac8 => "flac8",
            TrackFileType::Flac16 => "flac16",
            TrackFileType::Flac24 => "flac24",
            TrackFileType::Flac32 => "flac32",
            TrackFileType::Alac => "alac",
            TrackFileType::Mp3Cbr => "cbr",
            TrackFileType::Mp3Vbr => "vbr",
            TrackFileType::Aac => "aac",
            TrackFileType::Vorbis => "vorbis",
            TrackFileType::Opus => "opus",
            TrackFileType::Wavpack => "wavpack",
            TrackFileType::MonkeysAudio => "ape",
            TrackFileType::Unknown => "unknown",
        };
        write!(f, "{}", format_name)
    }
}

impl str::FromStr for TrackFileType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "flac" => Ok(TrackFileType::Flac),
            "flac4" => Ok(TrackFileType::Flac4),
            "flac8" => Ok(TrackFileType::Flac8),
            "flac16" => Ok(TrackFileType::Flac16),
            "flac24" => Ok(TrackFileType::Flac24),
            "flac32" => Ok(TrackFileType::Flac32),
            "alac" => Ok(TrackFileType::Alac),
            "cbr" => Ok(TrackFileType::Mp3Cbr),
            "vbr" => Ok(TrackFileType::Mp3Vbr),
            "aac" => Ok(TrackFileType::Aac),
            "vorbis" => Ok(TrackFileType::Vorbis),
            "opus" => Ok(TrackFileType::Opus),
            "wavpack" => Ok(TrackFileType::Wavpack),
            "ape" => Ok(TrackFileType::MonkeysAudio),
            _ => Ok(TrackFileType::Unknown),
        }
    }
}

#[derive(Debug)]
pub struct Track {
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub album_artists: Vec<String>,
    pub album: String,
    pub year: i32,
    pub track_number: i32,
    pub musicbrainz_track_id: Option<String>,
    pub has_front_cover: bool,
    pub front_cover_height: i32,
    pub front_cover_width: i32,
    pub bitrate: i32,
    pub sample_rate: i32,
    pub source: String,
    pub disc_number: i32,
    pub duration: i32,
    pub file_type: TrackFileType,
}