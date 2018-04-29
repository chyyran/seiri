#[derive(Debug, Primitive)]
pub enum TrackFileType {
    FLAC = 0,
    FLAC_4 = 1,
    FLAC_8 = 2,
    FLAC_16 = 3,
    FLAC_24 = 4,
    FLAC_32 = 5,

    // Unknown is randomly here for backwards-compat with the MP3 definitions...
    Unknown = 6,

    // The lossy range is [7, 11]
    MP3_CBR = 7,
    MP3_VBR = 8,
    AAC = 9,
    Vorbis = 10,
    Opus = 11,

    // The ALAC range is [12, 14]
    // Dummy for switching on.
    ALAC = 12,
    ALAC_16 = 13,
    ALAC_24 = 14,

    // AIFF is recommended over WAV due to support for ID3 over
    // RIFF frames. The range is [15, 20]
    AIFF = 15,
    /// 4-Bit AIFF. This is technically possible.
    AIFF_4 = 16,
    AIFF_8 = 17,
    AIFF_16 = 18,
    AIFF_24 = 19,
    AIFF_32 = 20,

    // Monkey's Audio range is [21, 24]
    MonkeysAudio = 21,
    MonkeysAudio_8 = 22,
    MonkeysAudio_16 = 23,
    MonkeysAudio_24 = 24,
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
    pub updated: String,
}
