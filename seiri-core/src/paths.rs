use std::fs;
use std::path::{PathBuf, Path};
use app_dirs::*;
use config::Config;
use rusqlite::Connection;
use database::{create_database, add_regexp_function, enable_wal_mode};
use toml;

/// Gets the application data path.
/// Panics if unable to be created.
pub fn get_appdata_path() -> PathBuf {
    let appdata_path = get_data_root(AppDataType::UserConfig)
        .ok()
        .and_then(|mut p: PathBuf| {
            p.push(".seiri");
            Some(p)
        })
        .unwrap();
    if let Err(_) = fs::create_dir_all(appdata_path.as_path()) {
        panic!("Unable to create application directory at {:?}", appdata_path)
    }
    appdata_path
}

fn write_default_config(path: &Path) -> Option<()> {
    let default_config = toml::to_string(&Config::default()).unwrap();
    fs::write(path.to_str().unwrap(), default_config).ok()
}

pub fn get_config() -> Config {
    let mut config_path = get_appdata_path();
    config_path.push("config.toml");
    if !config_path.exists() {
       if let None = write_default_config(config_path.as_path()) {
           panic!("Unable to write default configuration. Using default configuration.");
       }
    }
    let config_string = fs::read_to_string(config_path).unwrap();
    let config: Config = toml::from_str(&config_string).unwrap();
    config
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

pub fn ensure_music_folder(folder_path: &str) -> (PathBuf, PathBuf) {
    let music_folder = Path::new(folder_path);
    let mut music_folder = PathBuf::from(music_folder);
    let mut auto_add_folder = PathBuf::new();
    music_folder.clone_into(&mut auto_add_folder);
    auto_add_folder.pop();
    auto_add_folder.push("Automatically Add to Library");
    fs::create_dir_all(music_folder.as_path());
    fs::create_dir_all(auto_add_folder.as_path());
    (music_folder, auto_add_folder)
}