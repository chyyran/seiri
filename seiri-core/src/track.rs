extern crate serde_json;

use bangs::ticks_to_ms;
use error::{Error, Result};
use serde_json::value::Value;
use std::fmt;
use std::path::Path;
use std::str;
use taglibsharp;

#[derive(Debug, GraphQLEnum)]
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
            TrackFileType::Flac => 1,
            TrackFileType::Flac4 => 2,
            TrackFileType::Flac8 => 3,
            TrackFileType::Flac16 => 4,
            TrackFileType::Flac24 => 5,
            TrackFileType::Flac32 => 6,
            TrackFileType::Alac => 7,
            TrackFileType::Mp3Cbr => 8,
            TrackFileType::Mp3Vbr => 9,
            TrackFileType::Aac => 10,
            TrackFileType::Vorbis => 11,
            TrackFileType::Opus => 12,
            TrackFileType::Wavpack => 13,
            TrackFileType::MonkeysAudio => 14,
            TrackFileType::Unknown => 15,
        }
    }
}

impl From<i32> for TrackFileType {
    fn from(i: i32) -> Self {
        match i {
            1 => TrackFileType::Flac,
            2 => TrackFileType::Flac4,
            3 => TrackFileType::Flac8,
            4 => TrackFileType::Flac16,
            5 => TrackFileType::Flac24,
            6 => TrackFileType::Flac32,
            7 => TrackFileType::Alac,
            8 => TrackFileType::Mp3Cbr,
            9 => TrackFileType::Mp3Vbr,
            10 => TrackFileType::Aac,
            11 => TrackFileType::Vorbis,
            12 => TrackFileType::Opus,
            13 => TrackFileType::Wavpack,
            14 => TrackFileType::MonkeysAudio,
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
            "aac" => Ok(TrackFileType::Alac),
            "vorbis" => Ok(TrackFileType::Vorbis),
            "opus" => Ok(TrackFileType::Opus),
            "wavpack" => Ok(TrackFileType::Wavpack),
            "ape" => Ok(TrackFileType::MonkeysAudio),
            _ => Ok(TrackFileType::Unknown),
        }
    }
}

#[derive(Debug, GraphQLObject)]
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

impl Track {
    pub fn new(file_path: &Path, source: Option<&str>) -> Result<Track> {
        let json_data = taglibsharp::call_helper(file_path.to_str().unwrap());

        if let Err(err) = json_data {
            return Err(err);
        }

        let json_data = json_data.unwrap();
        let v: Value = serde_json::from_str(&json_data).unwrap();
        let title: &Value = &v["Title"];
        let artist: &Value = &v["Artist"];
        let album_artists: &Value = &v["AlbumArtists"];
        let album: &Value = &v["Album"];
        let year: &Value = &v["Year"];
        let track_number: &Value = &v["TrackNumber"];
        let disc_number: &Value = &v["DiscNumber"];
        let musicbrainz_track_id: &Value = &v["MusicBrainzTrackId"];
        let has_front_cover: &Value = &v["HasFrontCover"];
        let front_cover_height: &Value = &v["FrontCoverHeight"];
        let front_cover_width: &Value = &v["FrontCoverWidth"];
        let bitrate: &Value = &v["Bitrate"];
        let sample_rate: &Value = &v["SampleRate"];
        let duration: &Value = &v["Duration"];
        let file_type_str: &Value = &v["FileType"];

        let file_type = match file_type_str.as_str().unwrap() {
            "FLAC" => TrackFileType::Flac,
            "FLAC_4" => TrackFileType::Flac4,
            "FLAC_8" => TrackFileType::Flac8,
            "FLAC_16" => TrackFileType::Flac16,
            "FLAC_24" => TrackFileType::Flac24,
            "FLAC_32" => TrackFileType::Flac32,
            "ALAC" => TrackFileType::Alac,
            "MP3_CBR" => TrackFileType::Mp3Cbr,
            "MP3_VBR" => TrackFileType::Mp3Vbr,
            "AAC" => TrackFileType::Aac,
            "Vorbis" => TrackFileType::Vorbis,
            "Opus" => TrackFileType::Opus,
            "Wavpack" => TrackFileType::Wavpack,
            "MonkeysAudio" => TrackFileType::MonkeysAudio,
            _ => TrackFileType::Unknown,
        };

        let album_artists_unwrapped = match album_artists.as_array() {
            Some(arr) => arr,
            None => {
                return Err(Error::MissingRequiredTag(
                    file_path.to_str().unwrap().to_owned(),
                    "AlbumArtist",
                ))
            }
        };

        let title = title.as_str().unwrap_or("").to_owned();
        let artist = artist.as_str().unwrap_or("").to_owned();
        let album = album.as_str().unwrap_or("").to_owned();

        if title.is_empty() {
            return Err(Error::MissingRequiredTag(
                file_path.to_str().unwrap().to_owned(),
                "Title",
            ));
        }
        if artist.is_empty() {
            return Err(Error::MissingRequiredTag(
                file_path.to_str().unwrap().to_owned(),
                "Artist",
            ));
        }
        if album.is_empty() {
            return Err(Error::MissingRequiredTag(
                file_path.to_str().unwrap().to_owned(),
                "Album",
            ));
        }
        let track = Track {
            file_path: file_path.to_str().unwrap().to_owned(),
            source: source.unwrap_or("None").to_owned(),
            title: title,
            artist: artist,
            album_artists: album_artists_unwrapped
                .into_iter()
                .map(|val| val.as_str().unwrap().to_owned())
                .collect::<Vec<String>>(),
            album: album,
            year: year.as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap_or(0)
                .to_owned(),
            track_number: track_number
                .as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap_or(0)
                .to_owned(),
            disc_number: disc_number
                .as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap_or(0)
                .to_owned(),
            musicbrainz_track_id: if let &Value::String(ref string) = musicbrainz_track_id {
                Some(string.to_owned())
            } else {
                None
            },
            has_front_cover: has_front_cover.as_bool().unwrap().to_owned(),
            front_cover_height: front_cover_height
                .as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap_or(0)
                .to_owned(),
            front_cover_width: front_cover_width
                .as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap_or(0)
                .to_owned(),
            bitrate: bitrate
                .as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap()
                .to_owned(),
            sample_rate: sample_rate
                .as_i64()
                .and_then(|i| Some(i as i32))
                .unwrap()
                .to_owned(),
            duration: ticks_to_ms(duration.as_i64().unwrap().to_owned()),
            file_type: file_type,
        };

        Ok(track)
    }
}
