use anyhow::Result;
use reqwest::Url;

// THIS WILL BE REMOVED!
#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config {
        host: "aws.connect.psdb.cloud".into(),
        username: "zrhq79gia2vqhporjydc".into(),
        password: "pscale_pw_N11vup13sipUzd2cc8sY0nYxRp7WA0lEVfRydcizdwI".into(),
    };
    execute("SELECT * FROM counter", &config).await?;

    Ok(())
}

// TODO: args
async fn execute(query: &str, config: &Config) -> Result<()> {
    let url =
        Url::parse(format!("https://{}/psdb.v1alpha1.Database/Execute", config.host).as_str())
            .unwrap();

    // TODO: args
    let sql = ExecuteRequest {
        query: query.into(),
    };

    post(config, url.as_str(), sql).await?;
    Ok(())
}

async fn post<T>(config: &Config, url: &str, body: T) -> Result<()>
where
    T: serde::Serialize,
{
    let auth = format!("{}:{}", config.username, config.password);
    let auth = to_base64(&auth);
    println!("auth: {}", auth);

    let client = reqwest::Client::new();
    let req = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "database-js/1.7.0")
        .header("Authorization", format!("Basic {}", auth))
        .body(serde_json::to_string(&body).unwrap());
    let res = req.send().await.unwrap();

    println!("{:?}", res.text().await?);
    Ok(())
}

fn to_base64(s: &str) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(s.as_bytes())
}

#[derive(serde::Serialize)]
pub struct ExecuteRequest {
    pub query: String,
}

pub struct Config {
    pub host: String,
    pub username: String,
    pub password: String,
}
