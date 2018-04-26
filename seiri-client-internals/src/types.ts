
export interface ITrack {
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
    FLAC,
    FLAC4,
    FLAC8,
    FLAC16,
    FLAC24,
    FLAC32,
    ALAC,
    MP3_CBR,
    MP3_VBR,
    AAC,
    VORBIS,
    OPUS,
    WAVPACK,
    MONKEYS_AUDIO,
    UNKNOWN
  }