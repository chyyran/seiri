use std::env::home_dir;
use std::default::Default;
use serde_derive;

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