use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Primitive)]
pub enum TrackFileType {
    Unknown = 0,

    // For backwards compatibility purposes, the following have to hold
    // FLAC16 = 3, FLAC_32 = 5, CBR = 7, VBR = 8, AAC = 9.
    // This is mostly for my own personal usage, but 
    // that's all that really matters at this stage isn't it?

    // The FLAC range is [1, 6]
    // Dummy for switching on.
    FLAC4 = 1,
    FLAC8 = 2,
    FLAC16 = 3,
    FLAC24 = 4,
    FLAC32 = 5,
    FLAC = 6,

    // The lossy range is [7, 11]
    MP3CBR = 7,
    MP3VBR = 8,
    AAC = 9,
    Vorbis = 10,
    Opus = 11,

    // The Alac range is [12, 14]
    // Dummy for switching on.
    ALAC16 = 12,
    ALAC24 = 13,
    ALAC = 14,

    // Aiff is recommended over WAV due to support for ID3 over
    // RIFF frames. The range is [15, 20]
    /// 4-Bit Aiff. This is technically possible.
    AIFF4 = 15,
    AIFF8 = 16,
    AIFF16 = 17,
    AIFF24 = 18,
    AIFF32 = 19,
    AIFF = 20,

    // Monkey's Audio range is [21, 24]
    MonkeysAudio8 = 21,
    MonkeysAudio16 = 22,
    MonkeysAudio24 = 23,
    MonkeysAudio = 24,

    /// Generic for matching, this is not actually a valid return from katatsuki.
    MP3 = 780,
}

#[derive(Debug)]
pub struct Track {
    pub file_path: PathBuf,
    pub file_type: TrackFileType,
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
    pub updated: String,
}

impl FromStr for TrackFileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.to_lowercase().as_str() {
            "flac" => Ok(TrackFileType::FLAC),
            "flac4" => Ok(TrackFileType::FLAC4),
            "flac8" => Ok(TrackFileType::FLAC8),
            "flac16" => Ok(TrackFileType::FLAC16),
            "flac24" => Ok(TrackFileType::FLAC24),
            "flac32" => Ok(TrackFileType::FLAC32),
            "alac" => Ok(TrackFileType::ALAC),
            "alac16" => Ok(TrackFileType::ALAC16),
            "alac24" => Ok(TrackFileType::ALAC24),
            "cbr" => Ok(TrackFileType::MP3CBR),
            "vbr" => Ok(TrackFileType::MP3VBR),
            "aac" => Ok(TrackFileType::AAC),
            "vorbis" => Ok(TrackFileType::Vorbis),
            "opus" => Ok(TrackFileType::Opus),
            "aiff" => Ok(TrackFileType::AIFF),
            "aiff4" => Ok(TrackFileType::AIFF4),
            "aiff8" => Ok(TrackFileType::AIFF8),
            "aiff16" => Ok(TrackFileType::AIFF16),
            "aiff24" => Ok(TrackFileType::AIFF24),
            "aiff32" => Ok(TrackFileType::AIFF32),
            "ape" => Ok(TrackFileType::MonkeysAudio),
            "ape8" => Ok(TrackFileType::MonkeysAudio8),
            "ape16" => Ok(TrackFileType::MonkeysAudio16),
            "ape24" => Ok(TrackFileType::MonkeysAudio24),
            "mp3" => Ok(TrackFileType::MP3),
            _ => Ok(TrackFileType::Unknown),
        }
    }
}