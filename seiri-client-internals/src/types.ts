type UpdateDateFull =`${number}${number}${number}${number}-${number}${number}-${number}${number}`
type UpdateDateYear = `${number}${number}${number}${number}`
type UpdateDateMonth =`${number}${number}${number}${number}-${number}${number}`
type UpdateDate = UpdateDateFull | UpdateDateYear | UpdateDateMonth | "";

export interface Track {
  filePath: string;
  title: string;
  artist: string;
  albumArtists: string[];
  album: string;
  year: number;
  trackNumber: number;
  musicbrainzTrackId: string;
  hasFrontCover: boolean;
  frontCoverHeight: number;
  frontCoverWidth: number;
  bitrate: number;
  sampleRate: number;
  source: string;
  discNumber: number;
  duration: number;
  fileType: TrackFileType;
  updated: UpdateDate;
}

export enum TrackFileType {
  Unknown = 0,

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