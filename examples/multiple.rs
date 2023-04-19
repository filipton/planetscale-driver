use anyhow::Result;
use planetscale_driver::{Database, Deserializer, PSConnection, QueryBuilder};
use std::env::var;

#[derive(Database, Debug)]
pub struct TestDsadsa {
    pub id: u32,
    pub value: u32,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);

    QueryBuilder::new(
        "CREATE TABLE test_dsadsa(id INT AUTO_INCREMENT PRIMARY KEY, value INT NOT NULL)",
    )
    .execute(&conn)
    .await?;

    QueryBuilder::new(
        "INSERT INTO test_dsadsa(value) VALUES (69), (420), (1337), (69420), (1234), (1111)",
    )
    .execute(&conn)
    .await?;

    let res: Vec<TestDsadsa> = conn
        .execute("SELECT * FROM test_dsadsa")
        .await?
        .deserialize_multiple()?;
    println!("{:?}", res);

    conn.execute("DROP TABLE test_dsadsa").await?;
    return Ok(());
}
