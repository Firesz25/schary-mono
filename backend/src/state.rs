use sea_orm::{Database, DatabaseConnection};
use tera::Tera;

use crate::config::Config;
#[derive(Debug, Clone)]
pub struct State {
    pub db: DatabaseConnection,
    pub tmpl: Tera,
}

impl State {
    pub async fn new(cfg: Config) -> Self {
        let db = Database::connect(cfg.database_url).await.unwrap();
        Self {
            db,
            tmpl: Tera::new("templates/**/*").unwrap(),
        }
    }
}
