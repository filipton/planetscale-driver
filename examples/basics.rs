use anyhow::Result;
use planetscale_driver::{Database, Deserializer, PSConnection};
use std::env::var;

#[derive(Database, Debug)]
pub struct TestD {
    pub val: u32,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);

    let res = conn.execute("SELECT 1").await?;
    let res: TestD = res.deserialize()?;

    println!("{:?}", res);

    return Ok(());
}
