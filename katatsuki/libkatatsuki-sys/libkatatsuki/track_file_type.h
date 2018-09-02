
#pragma once
typedef enum track_file_type {
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
};

static inline const enum track_file_type get_flac_type(int bitdepth) {
    switch (bitdepth)
    {
        case 4:
            return track_file_type::FLAC4;
        case 8:
            return track_file_type::FLAC8;
        case 16:
            return track_file_type::FLAC16;
        case 24:
            return track_file_type::FLAC24;
        case 32:
            return track_file_type::FLAC32;
        default:
            return track_file_type::FLAC;
    }
};

static inline const enum track_file_type get_aiff_type(int bitdepth) {
    switch (bitdepth)
    {
        case 4:
            return track_file_type::AIFF4;
        case 8:
            return track_file_type::AIFF8;
        case 16:
            return track_file_type::AIFF16;
        case 24:
            return track_file_type::AIFF24;
        case 32:
            return track_file_type::AIFF32;
        default:
            return track_file_type::AIFF;
    }
};


static inline const enum track_file_type get_monkeys_audio_type(int bitdepth) {
    switch (bitdepth)
    {
        case 8:
            return track_file_type::MonkeysAudio8;
        case 16:
            return track_file_type::MonkeysAudio16;
        case 24:
            return track_file_type::MonkeysAudio24;
        default:
            return track_file_type::MonkeysAudio;
    }
};

static inline const enum track_file_type get_alac_type(int bitdepth) {
    switch (bitdepth)
    {
        case 16:
            return track_file_type::ALAC16;
        case 24:
            return track_file_type::ALAC24;
        default:
            return track_file_type::ALAC;
    }
};