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
        public int Bitrate { get; set; }
        public int SampleRate { get; set; }
        public uint DiscNumber { get; }
        public long Duration { get; set; }
        public IntPtr CoverBytes { get; set; }
        public CTrack(Track track) {
            this.Title = Marshal.StringToCoTaskMemUTF8(track.Title);
            this.Artist = Marshal.StringToCoTaskMemUTF8(track.Artist);
            this.AlbumArtists = Marshal.StringToCoTaskMemUTF8(String.Join(";", track.AlbumArtists));
            this.Album = Marshal.StringToCoTaskMemUTF8(track.Album);
            this.Year = track.Year;
            this.TrackNumber = track.TrackNumber;
            this.MusicBrainzTrackId = Marshal.StringToCoTaskMemUTF8(track.MusicBrainzTrackId);
            this.HasFrontCover = track.HasFrontCover;
            this.Bitrate = track.Bitrate;
            this.SampleRate = track.SampleRate;
            this.DiscNumber = track.DiscNumber;
            this.Duration = track.Duration;
            this.FileType = (uint)track.FileType;
            if (this.HasFrontCover) {
                this.CoverBytes = Marshal.AllocCoTaskMem(32);
                Marshal.Copy(track.CoverBytes, 0, this.CoverBytes, 32);
            } else {
                this.CoverBytes = IntPtr.Zero;
            }
        }
    }
}