using System;
using System.Runtime.InteropServices;

namespace libkatatsuki {

    [StructLayout(LayoutKind.Sequential)]
    public struct CTrack {
        public uint FileType { get; set; }
        public IntPtr Title { get; set; }
        public IntPtr Artist { get; set; }
        public IntPtr AlbumArtists { get; set; }
        public IntPtr Album { get; set; }
        public uint Year { get; set; }
        public uint TrackNumber { get; set; }
        public IntPtr MusicBrainzTrackId { get; set; }
        public bool HasFrontCover { get; set; }
        public int FrontCoverHeight { get; set; } 
        public int FrontCoverWidth { get; set; } 
        public int Bitrate { get; set; }
        public int SampleRate { get; set; }
        public uint DiscNumber { get; }
        public long Duration { get; set; }
        public CTrack(Track track) {
            this.Title = Marshal.StringToCoTaskMemUni(track.Title);
            this.Artist = Marshal.StringToCoTaskMemUni(track.Artist);
            this.AlbumArtists = Marshal.StringToCoTaskMemUni(String.Join(";", track.AlbumArtists));
            this.Album = Marshal.StringToCoTaskMemUni(track.Album);
            this.Year = track.Year;
            this.TrackNumber = track.TrackNumber;
            this.MusicBrainzTrackId = Marshal.StringToCoTaskMemUni(track.MusicBrainzTrackId);
            this.HasFrontCover = track.HasFrontCover;
            this.FrontCoverHeight = track.FrontCoverHeight;
            this.FrontCoverWidth = track.FrontCoverWidth;
            this.Bitrate = track.Bitrate;
            this.SampleRate = track.SampleRate;
            this.DiscNumber = track.DiscNumber;
            this.Duration = track.Duration;
            this.FileType = (uint)track.FileType;
        }
    }
}