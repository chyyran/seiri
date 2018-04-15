extern crate serde_json;

use error::{Error, Result};
use taglibsharp;
use std::path::PathBuf;
use serde_json::value::Value;

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
    pub fn value(&self) -> u32 {
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

pub struct Track {
    pub file_path: String,
    pub title: String,
    pub artist: String,
    pub album_artists: Vec<String>,
    pub album: String,
    pub year: u64,
    pub track_number: u64,
    pub musicbrainz_track_id: String,
    pub has_front_cover: bool,
    pub front_cover_height: u64,
    pub front_cover_width: u64,
    pub bitrate: u64,
    pub sample_rate: u64,
    pub source: String,
    pub duration: u64,
    pub file_type: TrackFileType,
}

impl Track {
    pub fn new(file_path: &PathBuf, source: Option<String>) -> Result<Track> {
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

        let empty_artists = Vec::<Value>::new();
        let album_artists_unwrapped = match album_artists.as_array() {
            Some(arr) => arr,
            None => &empty_artists,
        };

        let track = Track {
            file_path: file_path.to_str().unwrap().to_owned(),
            source: source.unwrap_or(String::from("None")),
            title: title.as_str().unwrap_or("Unknown Title").to_owned(),
            artist: artist.as_str().unwrap_or("Unknown Artist").to_owned(),
            album_artists: album_artists_unwrapped
                .into_iter()
                .map(|val| val.as_str().unwrap().to_owned())
                .collect::<Vec<String>>(),
            album: album.as_str().unwrap_or("Unknown Album").to_owned(),
            year: year.as_u64().unwrap_or(0).to_owned(),
            track_number: track_number.as_u64().unwrap_or(0).to_owned(),
            musicbrainz_track_id: musicbrainz_track_id.as_str().unwrap_or("").to_owned(),
            has_front_cover: has_front_cover.as_bool().unwrap().to_owned(),
            front_cover_height: front_cover_height.as_u64().unwrap_or(0).to_owned(),
            front_cover_width: front_cover_width.as_u64().unwrap_or(0).to_owned(),
            bitrate: bitrate.as_u64().unwrap().to_owned(),
            sample_rate: sample_rate.as_u64().unwrap().to_owned(),
            duration: duration.as_u64().unwrap().to_owned(),
            file_type: file_type,
        };

        Ok(track)
    }
}
