use anyhow::{Context, Result};
use deserializer::Database;
use reqwest::Url;
use structs::{Config, ExecuteRequest, ExecuteResponse};

mod structs;

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

// THIS WILL BE REMOVED!
#[tokio::main]
pub async fn main() -> Result<()> {
    let config: Config = Config {
        host: "aws.connect.psdb.cloud".into(),
        username: "zrhq79gia2vqhporjydc".into(),
        password: "pscale_pw_N11vup13sipUzd2cc8sY0nYxRp7WA0lEVfRydcizdwI".into(),
    };

    let res = execute("SELECT * FROM counter", &config).await?;
    let row: DeserializerT<Test> = res.into();
    println!("{:?}", row);

    //let rows: Vec<Test> = res.deserialize_multiple()?;
    //println!("{:?}", rows);

    Ok(())
}

#[derive(Debug)]
struct DeserializerT<T>(T);

impl<T> From<ExecuteResponse> for DeserializerT<T>
where
    T: Deserializer,
{
    fn from(res: ExecuteResponse) -> Self {
        let row: T = res.deserialize().unwrap();
        DeserializerT(row)
    }
}

// TODO: args
pub async fn execute(query: &str, config: &Config) -> Result<ExecuteResponse> {
    let url =
        Url::parse(format!("https://{}/psdb.v1alpha1.Database/Execute", config.host).as_str())
            .unwrap();

    // TODO: args
    let sql = ExecuteRequest {
        query: query.into(),
        session: None,
    };

    let res: ExecuteResponse = post(config, url.as_str(), sql).await?;
    Ok(res)
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

impl ExecuteResponse {
    pub fn deserialize<T>(&self) -> Result<T>
    where
        T: Deserializer,
    {
        if let Some(res) = &self.result {
            if let Some(_fields) = &res.fields {
                /*
                tmp.types = fields
                    .iter()
                    .map(|f| ParsedFieldType::from_str(&f.type_field))
                    .collect();
                */

                if res.rows.len() != 1 {
                    anyhow::bail!("Expected 1 row, got {}", res.rows.len());
                }

                let row = &res.rows[0];
                let row_str = from_base64(&row.values);
                let row_str = String::from_utf8(row_str).unwrap();

                let lengths: Vec<usize> = row
                    .lengths
                    .iter()
                    .map(|l| l.parse::<usize>().unwrap())
                    .collect();

                let mut row_vec: Vec<&str> = Vec::new();
                let mut last = 0;
                for length in lengths {
                    row_vec.push(&row_str[last..(last + length)]);
                    last += length;
                }

                let res = T::deserialize_raw(row_vec).context("Failed to deserialize row")?;
                return Ok(res);
            }
        }

        anyhow::bail!("No results found");
    }

    pub fn deserialize_multiple<T>(&self) -> Result<Vec<T>>
    where
        T: Deserializer,
    {
        if let Some(res) = &self.result {
            if let Some(_fields) = &res.fields {
                /*
                tmp.types = fields
                    .iter()
                    .map(|f| ParsedFieldType::from_str(&f.type_field))
                    .collect();
                */

                let mut out: Vec<T> = Vec::new();
                for row in &res.rows {
                    let row_str = from_base64(&row.values);
                    let row_str = String::from_utf8(row_str).unwrap();

                    let lengths: Vec<usize> = row
                        .lengths
                        .iter()
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect();

                    let mut row_vec: Vec<&str> = Vec::new();
                    let mut last = 0;
                    for length in lengths {
                        row_vec.push(&row_str[last..(last + length)]);
                        last += length;
                    }

                    out.push(T::deserialize_raw(row_vec).context("Failed to deserialize row")?);
                }

                return Ok(out);
            }
        }

        anyhow::bail!("No results found");
    }
}

fn to_base64(s: &str) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(s.as_bytes())
}

fn from_base64(s: &str) -> Vec<u8> {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.decode(s.as_bytes()).unwrap()
}
