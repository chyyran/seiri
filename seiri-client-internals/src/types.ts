
export interface Track {
    filePath: string;
    title: string;
    artist: string;
    albumArtists: string[];
    album: string;
    year: number;
    tracknumber: number;
    musicbrainzTrackId: string;
    hasFrontCover: boolean;
    frontCoverHeight: number;
    frontCoverWidth: number;
    bitrate: number;
    sampleRate: number;
    source: string;
    discnumber: number;
    duration: number;
    fileType: TrackFileType;
  }

  export enum TrackFileType {
    FLAC = "flac",
    FLAC4 = "flac4",
    FLAC8 = "flac8",
    FLAC16 = "flac16",
    FLAC24 = "flac24",
    FLAC32 = "flac32",
    ALAC = "alac",
    MP3_CBR = "cbr",
    MP3_VBR = "vbr",
    AAC = "aac",
    VORBIS = "vorbis",
    OPUS = "opus",
    WAVPACK = "wavpack",
    MONKEYS_AUDIO = "ape",
    UNKNOWN = "unknown"
  }