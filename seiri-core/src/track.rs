
use seiri::{Track, TrackFileType};
use seiri::ticks::ticks_to_ms;
use seiri::{Result, Error};
use serde_json::Value;
use serde_json::from_str;
use std::path::Path;
use chrono::prelude::*;

use taglibsharp;

pub trait TaglibTrack {
    fn new(file_path: &Path, source: Option<&str>) -> Result<Track>;
}

impl TaglibTrack for Track {
    fn new(file_path: &Path, source: Option<&str>) -> Result<Track> {

        let json_data = taglibsharp::call_helper(file_path.to_str().unwrap());

        if let Err(err) = json_data {
            return Err(err);
        }

        let json_data = json_data.unwrap();
        let v: Value = from_str(&json_data).unwrap();
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
            Some(arr) =>  { 
                if arr.len() != 0 {
                    arr
                } else {
                    return Err(Error::MissingRequiredTag(
                        file_path.to_str().unwrap().to_owned(),
                        "AlbumArtist",
                    ))
                }
                
            },
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
            updated: Local::now().format("%Y-%m-%d").to_string()
        };

        Ok(track)
    }
}