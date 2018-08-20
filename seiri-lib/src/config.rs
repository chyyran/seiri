use std::default::Default;
use std::path::Path;
use paths::*;
use std::fs;
use toml;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub music_folder: String,
}

impl Default for Config {
    fn default() -> Config {
        let mut home_dir = home_dir().unwrap();
        home_dir.push("Music");
        home_dir.push("seiri");
        Config { music_folder: home_dir.to_str().unwrap().to_owned() }
    }
}

fn write_default_config(path: &Path) -> Option<()> {
    let default_config = toml::to_string(&Config::default()).unwrap();
    fs::write(path.to_string_lossy().into_owned(), default_config).ok()
}

pub fn get_config() -> Config {
    let mut config_path = get_appdata_path();
    config_path.push("config.toml");
    if !config_path.exists() {
        if let None = write_default_config(config_path.as_path()) {
            eprintln!("CONFIGWRITEERR~Unable to write default configuration.");
            panic!("CONFIGWRITEERR~Unable to write default configuration.");
        }
    }

    // Should be safe to unwrap since 
    let config_string = fs::read_to_string(config_path).unwrap();
    let config = toml::from_str(&config_string);
    if let Ok(config) = config {
        config
    } else {
        eprintln!("CONFIGINVALID~Configuration file is in invalid format!");
        panic!("CONFIGWRITEERR~Unable to write default configuration.");
    }
}