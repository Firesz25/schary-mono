use sea_orm::{Database, DatabaseConnection};
use tera::Tera;
use redis::Client;

use crate::config::Config;
#[derive(Debug, Clone)]
pub struct State {
    pub db: DatabaseConnection,
    pub tmpl: Tera,
    pub redis: Client,
}

impl State {
    pub async fn new(cfg: Config) -> Self {
        let db = Database::connect(cfg.database_url).await.unwrap();
        Self {
            db,
            tmpl: Tera::new("templates/**/*").unwrap(),
            redis: Client::open(cfg.redis).unwrap()
        }
    }
}
