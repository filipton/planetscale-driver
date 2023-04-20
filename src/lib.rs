pub use crate::connections::PSConnection;
use crate::structs::VitessError;
pub use ps_driver_deserializer::Database;
pub use querybuilder::QueryBuilder;
pub use response::Deserializer;

use anyhow::Result;

mod connections;
mod querybuilder;
mod response;
mod structs;
mod utils;

// MAYBE ![CFG] THIS?
async fn post<B, R>(connection: &PSConnection, url: &str, body: B) -> Result<R>
where
    B: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    let body = serde_json::to_string(&body)?;
    post_raw(connection, url, body).await
}

async fn post_raw<R>(connection: &PSConnection, url: &str, body: String) -> Result<R>
where
    R: serde::de::DeserializeOwned,
{
    let req = connection
        .client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "database-rust/0.1.0")
        .header("Authorization", &connection.auth)
        .body(body);
    let res = req.send().await?;

    if !res.status().is_success() {
        let error: VitessError = serde_json::from_str(&res.text().await?)?;
        anyhow::bail!("Code: \"{}\", message: \"{}\"", error.code, error.message);
    }

    Ok(serde_json::from_str(&res.text().await?)?)
}
