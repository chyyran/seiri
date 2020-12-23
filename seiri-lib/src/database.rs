extern crate rusqlite;

use crate::bangs::{ms_to_ticks, ticks_to_ms, Bang};
use r2d2::{CustomizeConnection, Pool};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Error, Result, NO_PARAMS, functions::FunctionFlags};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use regex::Regex;
use rusqlite::types::ToSql;
use std::collections::HashMap;
use std::path::PathBuf;
use katatsuki::Track;
use katatsuki::TrackFileType;
use katatsuki::{ToPrimitive, FromPrimitive};
use crate::paths::get_appdata_path;

pub use rusqlite::Connection;

pub type ConnectionPool = Pool<SqliteConnectionManager>;

#[derive(Copy, Clone, Debug)]
struct SeiriConnectionCustomizer;
impl CustomizeConnection<Connection, Error> for SeiriConnectionCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> Result<()> {
        enable_wal_mode(conn).unwrap();
        add_regexp_function(conn).unwrap();
        create_database(conn);
        Ok(())
    }
}

pub fn get_database_connection() -> Connection {
    let mut database_path = get_appdata_path();
    database_path.push("tracks.db");
    let conn = Connection::open(database_path.as_path()).unwrap();
    enable_wal_mode(&conn).unwrap();
    add_regexp_function(&conn).unwrap();
    create_database(&conn);
    conn
}

pub fn get_connection_pool() -> ConnectionPool {
    let mut database_path = get_appdata_path();
    database_path.push("tracks.db");
    let manager = SqliteConnectionManager::file(&database_path);
    let pool = Pool::builder()
        .connection_customizer(Box::new(SeiriConnectionCustomizer))
        .build(manager)
        .unwrap();
    pool
}


fn escape_regex_search(string: &str) -> String {
    string.replace('\\', r"\\")
          .replace('?', r"\?")
          .replace('.', r"\.")
          .replace('+', r"\+")
          .replace('[', r"\[")
          .replace(']', r"\]")
          .replace('(', r"\(")
          .replace(')', r"\)")
          .replace('*', r"\*")
          .replace('^', r"\^")
}

#[allow(dead_code)]
pub fn add_regexp_function(db: &Connection) -> Result<()> {
    let mut cached_regexes = HashMap::new();
    db.create_scalar_function("regexp", 2, FunctionFlags::SQLITE_DETERMINISTIC, move |ctx| {
        let regex_s = ctx.get::<String>(0)?;
        let text = ctx.get::<String>(1)?;
        let entry = cached_regexes.entry(regex_s.clone());
        let regex = {
            use std::collections::hash_map::Entry::{Occupied, Vacant};
            match entry {
                Occupied(occ) => occ.into_mut(),
                Vacant(vac) => match Regex::new(&regex_s) {
                    Ok(r) => vac.insert(r),
                    Err(err) => {
                        return Err(Error::UserFunctionError(Box::new(err)));
                    }
                },
            }
        };

        let captures = regex.captures(&text);
        let capture_ok = captures
            .and_then(|capture| capture.get(1))
            .and_then(|m| Some(m.end() > 0))
            .unwrap_or(false);
        Ok(capture_ok)
    })
}

#[allow(dead_code)]
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
        FileType INTEGER,
        Updated DATE
    )",
        NO_PARAMS,
    ).unwrap();
}

#[allow(dead_code)]
pub fn enable_wal_mode(conn: &Connection) -> Result<()> {
    let mut statement = conn.prepare("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
    let _ = statement.query(NO_PARAMS)?;
    Ok(())
}

#[allow(dead_code)]
pub fn query_tracks(
    bang: Bang,
    conn: &Connection,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Track>> {
    let mut params = Vec::<(String, String)>::new();
    let mut query = if let Bang::All = bang {
        "SELECT * FROM tracks".to_string()
    } else {
        format!(
            "SELECT * FROM tracks WHERE ({})",
            to_query_string(bang, &mut params)
        )
    };

    if let Some(limit) = limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    if let Some(offset) = offset {
        query.push_str(&format!(" OFFSET {}", offset));
    }

    query.push_str(" ORDER BY CASE WHEN AlbumArtists = 'Various Artists' THEN 1 END, AlbumArtists,Album,TrackNumber");

    let mut tracks = Vec::<Track>::new();
    println!("Executing query: {:?}", query);
    let mut statement = conn.prepare(&query)?;
    println!("Preparing parameters: {:?}", params);

    let params = params
        .iter()
        .map(|c| (c.0.as_ref(), &c.1 as &dyn ToSql))
        .collect::<Vec<(&str, &dyn ToSql)>>();

    let mut rows = statement.query_named(params.as_slice())?;
    while let Ok(Some(row)) = rows.next() {
        let track = Track {
            file_path: PathBuf::from(&row.get::<_, String>(0)?),
            title: row.get(1)?,
            artist: row.get(2)?,
            album_artists: row.get::<_, String>(3)?
                .split(';')
                .map(|c| c.to_owned())
                .collect::<Vec<String>>(),
            album: row.get(4)?,
            year: row.get(5)?,
            track_number: row.get(6)?,
            musicbrainz_track_id: row.get(7).ok(),
            has_front_cover: row.get(8)?,
            front_cover_width: row.get(9).ok().unwrap_or(0),
            front_cover_height: row.get(10).ok().unwrap_or(0),
            bitrate: row.get(11)?,
            sample_rate: row.get(12)?,
            source: row.get(13).ok().unwrap_or("None".to_owned()),
            disc_number: row.get(14)?,
            duration: ticks_to_ms(row.get(15)?),
            file_type: TrackFileType::from_i32(row.get::<_, i32>(16)?)
                .unwrap_or(TrackFileType::Unknown),
            updated: row.get::<_, String>(17)?
        };
        tracks.push(track)
    }

    Ok(tracks)
}

#[allow(dead_code)]
fn get_rand_param() -> String {
    format!(":{}", thread_rng().sample_iter(&Alphanumeric).take(10).collect::<String>()).to_owned()
}

#[allow(dead_code)]
fn to_query_string(bang: Bang, params: &mut Vec<(String, String)>) -> String {
    match bang {
        Bang::FilePath(path) => {
            let param_name = get_rand_param();
            let format = format!("(FilePath = {})", param_name);
            params.push((param_name, format!("{}", path)));
            format
        }
        Bang::TitleSearch(title) => {
            let param_name = get_rand_param();
            let format = format!("(Title LIKE {})", param_name);
            params.push((param_name, format!("%{}%", title)));
            format
        }
        Bang::TitleSearchExact(title) => {
            let param_name = get_rand_param();
            let format = format!("(Title = {})", param_name);
            params.push((param_name, title));
            format
        }
        Bang::AlbumTitle(title) => {
            let param_name = get_rand_param();
            let format = format!("(Album LIKE {})", param_name);
            params.push((param_name, format!("%{}%", title)));
            format
        }
        Bang::AlbumTitleExact(title) => {
            let param_name = get_rand_param();
            let format = format!("(Album = {})", param_name);
            params.push((param_name, title));
            format
        }
        Bang::Artist(artist) => {
            let param_name = get_rand_param();
            let format = format!("(Artist LIKE {})", param_name);
            params.push((param_name, format!("%{}%", artist)));
            format
        }
        Bang::ArtistExact(artist) => {
            let param_name = get_rand_param();
            let format = format!("(Artist = {})", param_name);
            params.push((param_name, format!("{}", artist)));
            format
        }
        // todo: (Might want to make this smarter?)
        Bang::AlbumArtists(artist) => {
            let param_name = get_rand_param();
            let format = format!("(AlbumArtists REGEXP {})", param_name);
            params.push((
                param_name,
                format!("(?:^|;)(?:.*?)((?i){})(?:.*?)(?:;|$)", escape_regex_search(&artist)),
            ));
            format
        }
        Bang::AlbumArtistsExact(artist) => {
            let param_name = get_rand_param();
            let format = format!("(AlbumArtists REGEXP {})", param_name);
            params.push((param_name, format!("(?:^|;)({})(?:;|$)", escape_regex_search(&artist))));
            format
        }
        Bang::Source(source) => {
            let param_name = get_rand_param();
            let format = format!("(Source = {} COLLATE NOCASE)", param_name);
            params.push((param_name, format!("{}", source)));
            format
        }
        Bang::Format(filetype) => {

            match filetype {
                TrackFileType::FLAC => {
                    let param_name_lesser = get_rand_param();
                    let param_name_greater = get_rand_param();
                    let format = format!("(FileType BETWEEN {} AND {})", param_name_lesser, param_name_greater);
                    params.push((param_name_lesser, format!("{}", TrackFileType::FLAC4.to_i32().unwrap())));
                    params.push((param_name_greater, format!("{}", TrackFileType::FLAC.to_i32().unwrap())));
                    format
                }
                TrackFileType::AIFF => {
                    let param_name_lesser = get_rand_param();
                    let param_name_greater = get_rand_param();
                    let format = format!("(FileType BETWEEN {} AND {})", param_name_lesser, param_name_greater);
                    params.push((param_name_lesser, format!("{}", TrackFileType::AIFF4.to_i32().unwrap())));
                    params.push((param_name_greater, format!("{}", TrackFileType::AIFF.to_i32().unwrap())));
                    format
                }
                TrackFileType::ALAC => {
                    let param_name_lesser = get_rand_param();
                    let param_name_greater = get_rand_param();
                    let format = format!("(FileType BETWEEN {} AND {})", param_name_lesser, param_name_greater);
                    params.push((param_name_lesser, format!("{}", TrackFileType::ALAC16.to_i32().unwrap())));
                    params.push((param_name_greater, format!("{}", TrackFileType::ALAC.to_i32().unwrap())));
                    format
                }
                TrackFileType::MonkeysAudio => {
                    let param_name_lesser = get_rand_param();
                    let param_name_greater = get_rand_param();
                    let format = format!("(FileType BETWEEN {} AND {})", param_name_lesser, param_name_greater);
                    params.push((param_name_lesser, format!("{}", TrackFileType::MonkeysAudio8.to_i32().unwrap())));
                    params.push((param_name_greater, format!("{}", TrackFileType::MonkeysAudio.to_i32().unwrap())));
                    format
                }
                TrackFileType::MP3 => {
                    let param_name_lesser = get_rand_param();
                    let param_name_greater = get_rand_param();
                    let format = format!("(FileType = {} OR FileType = {})", param_name_lesser, param_name_greater);
                    params.push((param_name_lesser, format!("{}", TrackFileType::MP3CBR.to_i32().unwrap())));
                    params.push((param_name_greater, format!("{}", TrackFileType::MP3VBR.to_i32().unwrap())));
                    format
                }
                _ => {
                    let param_name = get_rand_param();
                    let format = format!("(FileType = {})", param_name);
                    params.push((param_name, format!("{}", filetype.to_i32().unwrap())));
                    format
                }
            }
        }
        Bang::BitrateLessThan(bitrate) => {
            let param_name = get_rand_param();
            let format = format!("(Bitrate < {})", param_name);
            params.push((param_name, format!("{}", bitrate)));
            format
        }
        Bang::BitrateGreaterThan(bitrate) => {
            let param_name = get_rand_param();
            let format = format!("(Bitrate > {})", param_name);
            params.push((param_name, format!("{}", bitrate)));
            format
        }
        Bang::CoverArtWidthGreaterThan(width) => {
            let param_name = get_rand_param();
            let format = format!("(FrontCoverWidth > {})", param_name);
            params.push((param_name, format!("{}", width)));
            format
        }
        Bang::CoverArtWidthLessThan(width) => {
            let param_name = get_rand_param();
            let format = format!("(FrontCoverWidth < {})", param_name);
            params.push((param_name, format!("{}", width)));
            format
        }
        Bang::CoverArtHeightGreaterThan(height) => {
            let param_name = get_rand_param();
            let format = format!("(FrontCoverHeight > {})", param_name);
            params.push((param_name, format!("{}", height)));
            format
        }
        Bang::CoverArtHeightLessThan(height) => {
            let param_name = get_rand_param();
            let format = format!("(FrontCoverHeight < {})", param_name);
            params.push((param_name, format!("{}", height)));
            format
        }
        Bang::DurationGreaterThan(duration) => {
            let param_name = get_rand_param();
            let format = format!("(Duration > {})", param_name);
            params.push((param_name, format!("{}", duration)));
            format
        }
        Bang::DurationLessThan(duration) => {
            let param_name = get_rand_param();
            let format = format!("(Duration < {})", param_name);
            params.push((param_name, format!("{}", duration)));
            format
        }
        Bang::UpdatedBefore(date) => {
            let param_name = get_rand_param();
            let format = format!("(Updated < {})", param_name);
            params.push((param_name, format!("{}", date)));
            format
        }
        Bang::UpdatedAfter(date) => {
            let param_name = get_rand_param();
            let format = format!("(Updated > {})", param_name);
            params.push((param_name, format!("{}", date)));
            format
        }
        Bang::HasCoverArt(has) => {
            let param_name = get_rand_param();
            let format = format!("(HasFrontCover = {})", param_name);
            params.push((param_name, format!("{}", has)));
            format
        }
        Bang::HasMusicbrainzId(has) => (if has {
            "(MusicBrainzTrackId IS NOT NULL)"
        } else {
            "(MusicBrainzTrackId IS NULL)"
        }).to_owned(),
        Bang::HasDuplicates(has) => (if has {
            "(Title, AlbumArtists) in (select Title, AlbumArtists from tracks group by Title, AlbumArtists having count(*) > 1)"
        } else {
            "(Title, AlbumArtists) not in (select Title, AlbumArtists from tracks group by Title, AlbumArtists having count(*) > 1)"
        }).to_owned(),
        Bang::FullTextSearch(search) => {
            let param_name = get_rand_param();
            let album_artists_param = get_rand_param();
            let format = format!("(Title LIKE {} OR Album LIKE {} OR Artist LIKE {} OR AlbumArtists REGEXP {} COLLATE NOCASE)", 
                param_name, param_name, param_name, album_artists_param);
            params.push((param_name, format!("%{}%", search)));
            params.push((
                album_artists_param,
                format!("(?:^|;)(?:.*?)((?i){})(?:.*?)(?:;|$)", escape_regex_search(&search)),
            ));

            format
        }
        Bang::FullTextSearchExact(search) => {
            let param_name = get_rand_param();
            let album_artists_param = get_rand_param();

            let format = format!(
                "(Title = {} OR Album = {} OR Artist = {} OR AlbumArtists REGEXP {} COLLATE NOCASE)",
                param_name, param_name, param_name, album_artists_param
            );
            params.push((param_name, format!("{}", search)));
            params.push((album_artists_param, format!("(?:^|;)({})(?:;|$)", escape_regex_search(&search))));
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
        // This should never happen, but we'll just give it a vacuous condition
        // To satisfy the compiler.
        Bang::All => "(FilePath = FilePath)".to_owned(),
    }
}

#[allow(dead_code)]
pub fn remove_track(track: &Track, conn: &Connection) {
    conn.execute(
        "DELETE FROM tracks WHERE FilePath = ?1",
        &[&track.file_path.to_string_lossy().into_owned()],
    ).unwrap();
}

#[allow(dead_code)]
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
                FileType,
                Updated) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7,
                        ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        &[
            &track.file_path.as_os_str().to_string_lossy().into_owned() as &dyn ToSql,
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
            &track.disc_number,
            &ms_to_ticks(track.duration),
            &track.file_type.to_i32().unwrap(),
            &track.updated,
        ],
    ).unwrap();
}
