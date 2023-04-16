use crate::structs::Session;

pub struct Config {
    pub host: String,
    pub username: String,
    pub password: String,
    pub session: Option<Session>,
    pub client: reqwest::Client,
}

impl Config {
    pub fn new(host: &str, username: &str, password: &str) -> Self {
        Self {
            host: host.into(),
            username: username.into(),
            password: password.into(),
            session: None,
            client: reqwest::Client::new(),
        }
    }
}
