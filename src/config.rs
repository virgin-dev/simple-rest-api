use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .expect("invalid configuration")
            .try_deserialize()
            .expect("invalid configuration")
    }
}