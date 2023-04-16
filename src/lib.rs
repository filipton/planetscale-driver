pub use config::Config;
pub use deserializer::Database;

use crate::structs::VitessError;
use anyhow::Result;
use reqwest::Url;
use structs::{ExecuteRequest, ExecuteResponse};
use utils::to_base64;

mod config;
mod response;
mod structs;
mod utils;

pub trait Deserializer {
    fn deserialize_raw(input: Vec<&str>) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Database, Debug)]
pub struct Test {
    pub id: i32,
    pub count: i32,
    pub elon: f64,
    pub test: String,
}

#[derive(Database, Debug)]
pub struct Count {
    pub count: i32,
}

pub async fn execute(query: &str, config: &mut Config) -> Result<ExecuteResponse> {
    let url =
        Url::parse(format!("https://{}/psdb.v1alpha1.Database/Execute", config.host).as_str())
            .unwrap();

    let sql = ExecuteRequest {
        query: query.into(),
        session: config.session.clone(),
    };

    let res: ExecuteResponse = post(config, url.as_str(), sql).await?;
    config.session = Some(res.session.clone());

    Ok(res)
}

async fn post<B, R>(config: &Config, url: &str, body: B) -> Result<R>
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
