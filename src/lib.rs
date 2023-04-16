pub use deserializer::Database;

use crate::structs::{Session, VitessError};
use anyhow::Result;
use reqwest::Url;
use structs::{ExecuteRequest, ExecuteResponse};
use utils::to_base64;

mod response;
mod structs;
mod utils;

pub trait Deserializer {
    fn deserialize_raw(input: Vec<&str>) -> Result<Self>
    where
        Self: Sized;
}

pub struct PSConnection {
    pub host: String,
    pub username: String,
    pub password: String,
    pub session: Option<Session>,
    pub client: reqwest::Client,
}

impl PSConnection {
    pub fn new(host: &str, username: &str, password: &str) -> Self {
        Self {
            host: host.into(),
            username: username.into(),
            password: password.into(),
            session: None,
            client: reqwest::Client::new(),
        }
    }

    pub async fn execute(&mut self, query: &str) -> Result<ExecuteResponse> {
        let url =
            Url::parse(format!("https://{}/psdb.v1alpha1.Database/Execute", self.host).as_str())
                .unwrap();

        let sql = ExecuteRequest {
            query: query.into(),
            session: self.session.clone(),
        };

        let res: ExecuteResponse = post(self, url.as_str(), sql).await?;
        self.session = Some(res.session.clone());

        Ok(res)
    }
}

// MAYBE ![CFG] THIS?
async fn post<B, R>(config: &PSConnection, url: &str, body: B) -> Result<R>
where
    B: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    let auth = format!("{}:{}", config.username, config.password);
    let auth = to_base64(&auth);

    let req = config
        .client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "database-js/1.7.0")
        .header("Authorization", format!("Basic {}", auth))
        .body(serde_json::to_string(&body)?);
    let res = req.send().await?;

    // CHECK IF RESPONSE IS ERROREED
    if !res.status().is_success() {
        let error: VitessError = serde_json::from_str(&res.text().await?)?;
        anyhow::bail!("Code: \"{}\", message: \"{}\"", error.code, error.message);
    }

    Ok(serde_json::from_str(&res.text().await?)?)
}
