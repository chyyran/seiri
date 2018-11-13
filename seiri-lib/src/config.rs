use dirs::home_dir;
use error::{ConfigErrorType, Error, Result};
use paths::*;
use std::default::Default;
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub music_folder: String,
}

impl Default for Config {
    fn default() -> Config {
        let mut home_dir = home_dir().unwrap();
        home_dir.push("Music");
        home_dir.push("seiri");
        Config {
            music_folder: home_dir.to_str().unwrap().to_owned(),
        }
    }
}

fn write_default_config(path: &Path) -> Option<()> {
    let default_config = toml::to_string(&Config::default()).unwrap();
    fs::write(path.to_string_lossy().into_owned(), default_config).ok()
}

pub fn get_config() -> Result<Config> {
    let mut config_path = get_appdata_path();
    config_path.push("config.toml");
    if !config_path.exists() && write_default_config(config_path.as_path()).is_none() {
        return Err(Error::ConfigError(ConfigErrorType::IOError(
            config_path.to_string_lossy().to_string(),
        )));
    }

    // Should be safe to unwrap since
    let config_string = fs::read_to_string(config_path).unwrap();
    let config = toml::from_str(&config_string);
    if let Ok(config) = config {
        Ok(config)
    } else {
        Err(Error::ConfigError(ConfigErrorType::Invalid))
    }
}
