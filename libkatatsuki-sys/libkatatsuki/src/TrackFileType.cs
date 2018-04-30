using System;

namespace libkatatsuki
{
    public enum TrackFileType : uint
    {

        Unknown = 0,
          // The FLAC range is [1, 6]
        # region FLAC
        // Dummy for switching on.
        FLAC_4 = 1,
        FLAC_8 = 2,
        FLAC_16 = 3,
        FLAC_24 = 4,
        FLAC_32 = 5,        
        FLAC = 6,
        # endregion

        // The lossy range is [7, 11]
        # region Lossy
        MP3_CBR = 7,
        MP3_VBR = 8,
        AAC = 9,
        Vorbis = 10,
        Opus = 11,
        # endregion

        // The ALAC range is [12, 14]
        # region ALAC
        // Dummy for switching on.
        ALAC_16 = 12,
        ALAC_24 = 13,
        ALAC = 14,
        # endregion

        // AIFF is recommended over WAV due to support for ID3 over
        // RIFF frames. The range is [15, 20]
        # region AIFF

        /// 4-Bit AIFF. This is technically possible.
        AIFF_4 = 15,
        AIFF_8 = 16,
        AIFF_16 = 17,
        AIFF_24 = 18,
        AIFF_32 = 19,
        AIFF = 20,
        # endregion AIFF

        // Monkey's Audio range is [21, 24]
        # region MonkeysAudio
        MonkeysAudio_8 = 21,
        MonkeysAudio_16 = 22,
        MonkeysAudio_24 = 23,        
        MonkeysAudio = 24,
        # endregion MonkeysAudio
    }

    public class TrackFileTypeGetter
    {

        public static TrackFileType GetMp3Type(string description)
        {
            if (description.EndsWith("Layer 3 VBR",
               StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.MP3_VBR;
            }
            if (description.EndsWith("Layer 3", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.MP3_CBR;
            }
            return TrackFileType.Unknown;
        }


        public static TrackFileType GetAiffType(int bitdepth)
        {
            switch (bitdepth)
            {
                case 4:
                    return TrackFileType.AIFF_4;
                case 8:
                    return TrackFileType.AIFF_8;
                case 16:
                    return TrackFileType.AIFF_16;
                case 24:
                    return TrackFileType.AIFF_24;
                case 32:
                    return TrackFileType.AIFF_32;
                default:
                    return TrackFileType.AIFF;
            }
        }

        public static TrackFileType GetMonkeysAudioType(int bitdepth)
        {
            switch (bitdepth)
            {
                case 8:
                    return TrackFileType.MonkeysAudio_8;
                case 16:
                    return TrackFileType.MonkeysAudio_16;
                case 24:
                    return TrackFileType.MonkeysAudio_24;
                default:
                    return TrackFileType.MonkeysAudio;
            }
        }
        public static TrackFileType GetAlacType(int bitdepth)
        {
            switch (bitdepth)
            {
                case 16:
                    return TrackFileType.ALAC_16;
                case 24:
                    return TrackFileType.ALAC_24;
                default:
                    return TrackFileType.ALAC;
            }
        }
        public static TrackFileType GetFlacType(int bitdepth)
        {
            switch (bitdepth)
            {
                case 4:
                    return TrackFileType.FLAC_4;
                case 8:
                    return TrackFileType.FLAC_8;
                case 16:
                    return TrackFileType.FLAC_16;
                case 24:
                    return TrackFileType.FLAC_24;
                case 32:
                    return TrackFileType.FLAC_32;
                default:
                    return TrackFileType.FLAC;
            }
        }
    }
}