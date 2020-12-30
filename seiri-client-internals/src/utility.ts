
import { TrackFileType } from "./types";

export const fileTypeString = (fileType: TrackFileType) => {
    switch (fileType) {
        case TrackFileType.FLAC:
            return "FLAC";
        case TrackFileType.FLAC4:
            return "FLAC (4-bit)";
        case TrackFileType.FLAC8:
            return "FLAC (8-bit)";
        case TrackFileType.FLAC16:
            return "FLAC (16-bit)";
        case TrackFileType.FLAC24:
            return "FLAC (24-bit Hi-Res)";
        case TrackFileType.FLAC32:
            return "FLAC (32-bit Integral)";
        case TrackFileType.MP3CBR:
            return "MP3 (Constant Bitrate)";
        case TrackFileType.MP3VBR:
            return "MP3 (Variable Bitrate)";
        case TrackFileType.AAC:
            return "AAC (M4A Audio)";
        case TrackFileType.ALAC:
            return "Apple Lossless";
        case TrackFileType.ALAC16:
            return "Apple Lossless (16-bit)";
        case TrackFileType.ALAC24:
            return "Apple Lossless (24-bit Hi-Res)";
        case TrackFileType.AIFF:
            return "AIFF (PCM Audio)";
        case TrackFileType.AIFF4:
            return "AIFF (4-bit PCM)"
        case TrackFileType.AIFF8:
            return "AIFF (8-bit PCM)"
        case TrackFileType.AIFF16:
            return "AIFF (16-bit PCM)"
        case TrackFileType.AIFF24:
            return "AIFF (24-bit PCM)"
        case TrackFileType.AIFF32:
            return "AIFF (32-bit PCM)"
        case TrackFileType.Opus:
            return "Opus";
        case TrackFileType.Vorbis:
            return "Vorbis";
        case TrackFileType.MonkeysAudio:
            return "Monkey's Audio";
        case TrackFileType.MonkeysAudio16:
            return "Monkey's Audio (16-bit)";
        case TrackFileType.MonkeysAudio24:
            return "Monkey's Audio (24-bit)";
        case TrackFileType.Unknown:
            return "Unknown";
        default:
            return "";
    }
}

export const msToTime = (ms: number) => {
  const minutes = Math.floor(ms / 60000);
  const seconds = ((ms % 60000) / 1000).toFixed(0);
  return minutes + ":" + (Number(seconds) < 10 ? "0" : "") + seconds;
}
