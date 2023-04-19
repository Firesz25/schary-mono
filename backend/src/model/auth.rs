use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub login: String,
    pub password: String,
}
