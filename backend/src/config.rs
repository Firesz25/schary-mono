use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_redis")]
    pub redis: String,
}

impl Config {
    pub fn new() -> Self {
        serde_yaml::from_str(std::fs::read_to_string("Config.yaml").unwrap().as_str()).unwrap()
    }
}

fn default_redis() -> String {
    "redis://127.0.0.1".to_owned()
}
