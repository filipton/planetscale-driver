use anyhow::Result;
use planetscale_driver::{query, Database, Deserializer, PSConnection};
use std::env::var;

#[derive(Database, Debug)]
pub struct TestD {
    pub val: u32,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);

    let res: TestD = query("SELECT 1").fetch_one(&mut conn).await?;
    println!("{:?}", res);

    return Ok(());
}
