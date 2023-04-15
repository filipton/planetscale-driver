use anyhow::Result;
use reqwest::Url;
use structs::{Config, ExecuteRequest, ExecuteResponse};

mod structs;

// THIS WILL BE REMOVED!
#[tokio::main]
pub async fn main() -> Result<()> {
    let config: Config = Config {
        host: "aws.connect.psdb.cloud".into(),
        username: "zrhq79gia2vqhporjydc".into(),
        password: "pscale_pw_N11vup13sipUzd2cc8sY0nYxRp7WA0lEVfRydcizdwI".into(),
    };
    execute("SELECT * FROM counter", &config).await?;

    Ok(())
}

// TODO: args
pub async fn execute(query: &str, config: &Config) -> Result<()> {
    let url =
        Url::parse(format!("https://{}/psdb.v1alpha1.Database/Execute", config.host).as_str())
            .unwrap();

    // TODO: args
    let sql = ExecuteRequest {
        query: query.into(),
        session: None,
    };

    let res: ExecuteResponse = post(config, url.as_str(), sql).await?;
    println!("res: {:?}", res);

    Ok(())
}

async fn post<B, R>(config: &Config, url: &str, body: B) -> Result<R>
where
    B: serde::Serialize,
    R: serde::de::DeserializeOwned,
{
    let auth = format!("{}:{}", config.username, config.password);
    let auth = to_base64(&auth);

    let client = reqwest::Client::new();
    let req = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "database-js/1.7.0")
        .header("Authorization", format!("Basic {}", auth))
        .body(serde_json::to_string(&body)?);
    let res = req.send().await?;

    // CHECK IF RESPONSE IS ERROREED
    // throw with anyhow! macro

    Ok(serde_json::from_str(&res.text().await?)?)
}

fn to_base64(s: &str) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(s.as_bytes())
}
