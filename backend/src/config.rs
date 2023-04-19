use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        serde_yaml::from_str(std::fs::read_to_string("Config.yaml").unwrap().as_str()).unwrap()
    }
}
