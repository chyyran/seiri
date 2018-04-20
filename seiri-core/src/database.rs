extern crate rusqlite;

use track::TrackFileType;
use rusqlite::types::ToSql;
use rusqlite::Connection;
use track::Track;
use bangs::Bang;
use rand::{thread_rng, Rng};
use rusqlite::Result;

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

pub fn query_tracks(bang: Bang, conn: &Connection) -> Result<Vec<Track>> {
    let mut params = Vec::<(String, String)>::new();
    let query = if let Bang::All = bang {
        "SELECT * FROM tracks".to_string()
    } else {
        format!(
            "SELECT * FROM tracks WHERE ({})",
            to_query_string(bang, &mut params)
        )
    };

    let mut tracks = Vec::<Track>::new();
    println!("Executing query: {:?}", query);
    let mut statement = conn.prepare(&query)?;
    println!("Preparing parameters: {:?}", params);

    let params = params
        .iter()
        .map(|c| (c.0.as_ref(), &c.1 as &ToSql))
        .collect::<Vec<(&str, &ToSql)>>();

    let mut rows = statement.query_named(params.as_slice())?;
    while let Some(Ok(row)) = rows.next() {
        let track = Track {
            file_path: row.get_checked(0)?,
            title: row.get_checked(1)?,
            artist: row.get_checked(2)?,
            album_artists: row.get_checked::<_, String>(3)?
                .split(';')
                .map(|c| c.to_owned())
                .collect::<Vec<String>>(),
            album: row.get_checked(4)?,
            year: row.get_checked(5)?,
            track_number: row.get_checked(6)?,
            musicbrainz_track_id: row.get_checked(7).ok(),
            has_front_cover: row.get_checked(8)?,
            front_cover_width: row.get_checked(9).ok().unwrap_or(0),
            front_cover_height: row.get_checked(10).ok().unwrap_or(0),
            bitrate: row.get_checked(11)?,
            sample_rate: row.get_checked(12)?,
            source: row.get_checked(13).ok().unwrap_or("None".to_owned()),
             //Skip DiscNumber
            duration: row.get_checked(15)?,
            file_type: TrackFileType::from(row.get_checked::<_, u32>(16)?)
        };
        tracks.push(track)
    }
  
    Ok(tracks)
}

fn get_rand_param() -> String {
    let mut rng = thread_rng();
    format!(":{}", rng.gen_ascii_chars().take(10).collect::<String>()).to_owned()
}

fn to_query_string(bang: Bang, params: &mut Vec<(String, String)>) -> String {
    match bang {
        Bang::TitleSearch(title) => {
            let param_name = get_rand_param();
            let format = format!("(Title LIKE {})", param_name);
            params.push((param_name, format!("%{}%",title)));
            format
        }
        Bang::TitleSearchExact(title) => {
            let param_name = get_rand_param();
            let format = format!("(Title = {})", param_name);
            params.push((param_name, title));
            format
        }
        Bang::Artist(artist) => {
            let param_name = get_rand_param();
            let format = format!("(Artist LIKE {})", param_name);
            params.push((param_name, format!("%{}%",artist)));
            format
        }
        Bang::LogicalAnd(lhs, rhs) => {
            let lhs = to_query_string(*lhs, params);
            let rhs = to_query_string(*rhs, params);
            format!("({}) AND ({})", lhs, rhs)
        }
        Bang::LogicalOr(lhs, rhs) => {
            let lhs = to_query_string(*lhs, params);
            let rhs = to_query_string(*rhs, params);
            format!("({}) OR ({})", lhs, rhs)
        }
        Bang::Grouping(bang) => {
            let bang = to_query_string(*bang, params);
            format!("({})", bang)
        }
        _ => "".to_string(),
    }
}

pub fn remove_track(track: &Track, conn: &Connection) {
    conn.execute(
        "DELETE FROM tracks WHERE FilePath = ?1",
        &[&track.file_path],
    ).unwrap();
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
