using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using TagLib;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.PixelFormats;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;

namespace taglibsharp_katatsuki
{
    public class Track
    {
        public string Title { get; set; }
        public string Artist { get; set; }
        public IList<string> AlbumArtists { get; set; }
        public string Album { get; set; }
        public uint Year { get; set; }
        public uint TrackNumber { get; set; }
        public string MusicBrainzTrackId { get; set; }
        public bool HasFrontCover { get; set; }
        public int FrontCoverHeight { get; set; } = 0;
        public int FrontCoverWidth { get; set; } = 0;
        public int Bitrate { get; set; }
        public int SampleRate { get; set; }
	public uint DiscNumber { get; }
        public long Duration { get; set; }

        [JsonConverter(typeof(StringEnumConverter))]
        public TrackFileType FileType { get; set; }

        public Track(string filename)
        {
            using (var file = TagLib.File.Create(filename))
            {
                this.FileType = Track.GetTrackFileType(file.Properties.Description, file.Properties.BitsPerSample);
                this.Duration = file.Properties.Duration.Ticks;
                this.SampleRate = file.Properties.AudioSampleRate;
                this.Bitrate = file.Properties.AudioBitrate;
                this.Album = file.Tag.Album;
                this.AlbumArtists = file.Tag.AlbumArtists.Select(s => s.Trim()).ToList();
                this.Artist = file.Tag.FirstPerformer;
                this.TrackNumber = file.Tag.Track;
                this.Year = file.Tag.Year;
                this.MusicBrainzTrackId = file.Tag.MusicBrainzTrackId;
                this.Title = file.Tag.Title;
		this.DiscNumber = file.Tag.Disc == 0 ? 1 : file.Tag.Disc;
                var frontAlbum = from picture in file.Tag.Pictures
                                 where picture.Type == TagLib.PictureType.FrontCover
                                 select picture;
                this.HasFrontCover = frontAlbum.Any();
                if (this.HasFrontCover)
                {
                    using (Image<Rgba32> image = Image.Load(new MemoryStream(frontAlbum.First().Data.Data)))
                    {
                        this.FrontCoverHeight = image.Height;
                        this.FrontCoverWidth = image.Width;
                    }
                }
            }

        }
        private static TrackFileType GetTrackFileType(string description, int bitdepth = 0)
        {

            if (description.Equals("Flac Audio", StringComparison.InvariantCultureIgnoreCase))
            {
                return Track.GetFlacType(bitdepth);
            }
            if (description.StartsWith("MPEG Version",
             StringComparison.InvariantCultureIgnoreCase))
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
            }
            if (description.Equals("MPEG-4 Audio (alac)", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.ALAC;
            }
            if (description.Equals("MPEG-4 Audio (mp4a)", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.AAC;
            }
            if (description.StartsWith("Monkey's Audio APE", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.MonkeysAudio;
            }
            if (description.StartsWith("WavPack", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.Wavpack;
            }
            if (description.StartsWith("Opus", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.Opus;
            }
            if (description.StartsWith("Vorbis", StringComparison.InvariantCultureIgnoreCase))
            {
                return TrackFileType.Vorbis;
            }
            return TrackFileType.Unknown;
        }
        private static TrackFileType GetFlacType(int bitdepth)
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