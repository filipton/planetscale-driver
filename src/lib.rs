use std::fmt::Debug;
use std::str::FromStr;

use anyhow::{Context, Result};
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
    let res = execute("SELECT * FROM counter", &config).await?;
    //println!("{:?}", res);
    let rows = res.get_rows();
    println!("{:?}", rows);
    println!("{:?}", res.get_rows().parse_value::<f64>(0, 2));
    println!("{:?}", rows.rows[0].parse_value::<u32>(1));

    Ok(())
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
    pub fn get_rows(&self) -> RowsResultValue {
        let mut tmp = RowsResultValue {
            types: vec![],
            rows: vec![],
        };

        if let Some(res) = &self.result {
            if let Some(fields) = &res.fields {
                tmp.types = fields
                    .iter()
                    .map(|f| ParsedFieldType::from_str(&f.type_field))
                    .collect();

                for row in &res.rows {
                    let mut tmp_row: Vec<RowValue> = vec![];
                    let raw_row = from_base64(&row.values);
                    let lengths: Vec<usize> = row
                        .lengths
                        .iter()
                        .map(|l| l.parse::<usize>().unwrap())
                        .collect();

                    let mut last = 0;
                    for length in lengths {
                        tmp_row.push(raw_row[last..(last + length)].to_vec());
                        last += length;
                    }
                    tmp.rows.push(tmp_row);
                }
            }
        }

        tmp
    }
}

impl RowsResultValue {
    pub fn parse_value<T>(&self, row: usize, index: usize) -> Result<T>
    where
        T: std::str::FromStr,
        <T as FromStr>::Err: Debug,
    {
        //let row_type = self.types.get(index).expect("Column index out of bounds");
        let row = self.rows.get(row).context("Row index out of bounds")?;
        let value = row.get(index).context("Column index out of bounds")?;
        let value_str = std::str::from_utf8(value).context("Value is not utf8")?;

        value_str
            .parse::<T>()
            .map_err(|e| anyhow::anyhow!("{:?}", e))
    }
}

trait RowValueParser {
    fn parse_value<T>(&self, index: usize) -> Result<T>
    where
        T: std::str::FromStr,
        <T as FromStr>::Err: Debug;
}

impl RowValueParser for Vec<RowValue> {
    fn parse_value<T>(&self, index: usize) -> Result<T>
    where
        T: std::str::FromStr,
        <T as FromStr>::Err: Debug,
    {
        let value = self.get(index).context("Column index out of bounds")?;
        let value_str = std::str::from_utf8(value).context("Value is not utf8")?;

        value_str
            .parse::<T>()
            .map_err(|e| anyhow::anyhow!("{:?}", e))
    }
}

/// In rows Vec<u8> is in utf8
#[derive(Debug, Clone)]
pub struct RowsResultValue {
    pub types: Vec<ParsedFieldType>,
    pub rows: Vec<Vec<RowValue>>,
}
pub type RowValue = Vec<u8>;

#[derive(Debug, Clone)]
pub enum ParsedFieldType {
    INT8,
    INT16,
    INT24,
    INT32,
    UINT8,
    UINT16,
    UINT24,
    UINT32,
    YEAR,
    FLOAT32,
    FLOAT64,
    DECIMAL,
    INT64,
    UINT64,
    DATE,
    TIME,
    DATETIME,
    TIMESTAMP,
    BLOB,
    BIT,
    VARBINARY,
    BINARY,
    JSON,
    TEXT,
    DEFAULT,
}

impl ParsedFieldType {
    pub fn from_str(s: &str) -> ParsedFieldType {
        match s {
            "INT8" => ParsedFieldType::INT8,
            "INT16" => ParsedFieldType::INT16,
            "INT24" => ParsedFieldType::INT24,
            "INT32" => ParsedFieldType::INT32,
            "UINT8" => ParsedFieldType::UINT8,
            "UINT16" => ParsedFieldType::UINT16,
            "UINT24" => ParsedFieldType::UINT24,
            "UINT32" => ParsedFieldType::UINT32,
            "YEAR" => ParsedFieldType::YEAR,
            "FLOAT32" => ParsedFieldType::FLOAT32,
            "FLOAT64" => ParsedFieldType::FLOAT64,
            "DECIMAL" => ParsedFieldType::DECIMAL,
            "INT64" => ParsedFieldType::INT64,
            "UINT64" => ParsedFieldType::UINT64,
            "DATE" => ParsedFieldType::DATE,
            "TIME" => ParsedFieldType::TIME,
            "DATETIME" => ParsedFieldType::DATETIME,
            "TIMESTAMP" => ParsedFieldType::TIMESTAMP,
            "BLOB" => ParsedFieldType::BLOB,
            "BIT" => ParsedFieldType::BIT,
            "VARBINARY" => ParsedFieldType::VARBINARY,
            "BINARY" => ParsedFieldType::BINARY,
            "JSON" => ParsedFieldType::JSON,
            "TEXT" => ParsedFieldType::TEXT,
            _ => ParsedFieldType::DEFAULT,
        }
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
