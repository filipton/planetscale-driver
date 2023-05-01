use anyhow::Result;
use planetscale_driver::{query, Database, DatabaseJSON, PSConnection};
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Database, Debug)]
pub struct TestD {
    pub val: u32,
    pub test: TestJSON,
}

#[derive(DatabaseJSON, Deserialize, Serialize, Debug)]
pub struct TestJSON {
    pub text: String,
    pub test: u32,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);

    let json: TestJSON = TestJSON {
        text: "test1234321".to_string(),
        test: 1234,
    };

    let res: TestD = query("SELECT 1010, '$0'").bind(json).fetch_one(&mut conn).await?;
    println!("{:?}", res);

    return Ok(());
}
