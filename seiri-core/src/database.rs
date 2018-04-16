extern crate rusqlite;

use rusqlite::Connection;
use track::Track;

pub fn create_database(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks ( 
        FilePath TEXT PRIMARY KEY,
        Title TEXT,
        Artist TEXT,
        AlbumArtists TEXT,
        Album TEXT,
        Year INTEGER,
        TrackNumber INTEGER, 
        MusicBrainzTrackId TEXT,
        HasFrontCover INTEGER,
        FrontCoverWidth INTEGER,
        FrontCoverHeight INTEGER,
        Bitrate INTEGER,
        SampleRate INTEGER,
        Source TEXT,
        DiscNumber INTEGER,
        Duration INTEGER,
        FileType INTEGER
    )",
        &[],
    ).unwrap();
}

pub fn remove_track(track: &Track, conn: &Connection) {
    conn.execute("DELETE FROM tracks WHERE FilePath = ?1", &[&track.file_path]).unwrap();
}

pub fn add_track(track: &Track, conn: &Connection) {
    conn.execute(
        "INSERT OR REPLACE INTO tracks(
                FilePath, 
                Title,
                Artist,
                AlbumArtists,
                Album,
                Year,
                TrackNumber,
                MusicBrainzTrackId,
                HasFrontCover,
                FrontCoverWidth,
                FrontCoverHeight, 
                Bitrate,
                SampleRate,
                Source,
                DiscNumber,
                Duration,
                FileType) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7,
                        ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        &[
            &track.file_path,
            &track.title,
            &track.artist,
            &track.album_artists.join(";"),
            &track.album,
            &track.year,
            &track.track_number,
            &track.musicbrainz_track_id,
            &track.has_front_cover,
            &track.front_cover_width,
            &track.front_cover_height,
            &track.bitrate,
            &track.sample_rate,
            &track.source,
            &0,
            &track.duration,
            &track.file_type.value(),
        ],
    ).unwrap();
}
