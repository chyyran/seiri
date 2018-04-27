use app_dirs::*;
use database::{add_regexp_function, create_database, enable_wal_mode};
use rusqlite::{Connection};
use std::path::PathBuf;
use std::fs;
use std::io::Result;

/// Gets the application data path.
/// Panics if unable to be created.
pub fn get_appdata_path() -> Result<PathBuf> {
    let appdata_path = get_data_root(AppDataType::UserConfig)
        .ok()
        .and_then(|mut p: PathBuf| {
            p.push(".seiri");
            Some(p)
        })
        .unwrap();
    if let Err(err) = fs::create_dir_all(appdata_path.as_path()) {
        Err(err)
    } else {
        Ok(appdata_path)
    }
}

pub fn get_database_connection() -> Connection {
    let mut database_path = get_appdata_path().unwrap();
    database_path.push("tracks.db");
    let conn = Connection::open(database_path.as_path()).unwrap();
    enable_wal_mode(&conn).unwrap();
    add_regexp_function(&conn).unwrap();
    create_database(&conn);
    conn
}