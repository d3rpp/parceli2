// this function is run before tokio is created
// so we need to use std lib functions for file system operations
use directories::UserDirs;
use std::fs::read_to_string;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
}

impl Config {
    #[must_use]
    pub fn load() -> Config {
        let config_dir = UserDirs::new()
            .expect("Unknown User")
            .home_dir()
            .join(".parceli.toml");

        if let Ok(file) = read_to_string(config_dir) {
            if let Ok(config) = toml::from_str::<Config>(file.as_str()) {
                config
            } else {
                Config::default()
            }
        } else {
            Config::default()
        }
    }
}
