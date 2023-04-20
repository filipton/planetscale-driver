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
    let mut conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);

    QueryBuilder::new(
        "CREATE TABLE test_dsadsa2(id INT AUTO_INCREMENT PRIMARY KEY, value INT NOT NULL)",
        &mut conn,
    )
    .execute()
    .await?;

    conn.execute("INSERT INTO test_dsadsa2(value) VALUES (321), (654)")
        .await?;

    let q_correct = QueryBuilder::new(
        "INSERT INTO test_dsadsa2(value) VALUES (69), (420), (1337), (69420), (1234), (1111)", &mut conn
    );
    let q_wrong = QueryBuilder::new(
        "INSERT INTO test_dsadsa2(valueccxzcxzcxz) VALUES (69), (420), (1337), (69420), (1234), (1111)", &mut conn
    );

    // Intentionally wrong query without catching the error
    _ = conn.transaction(vec![q_correct, q_wrong]).await;

    let res: Vec<TestDsadsa> = conn
        .execute("SELECT * FROM test_dsadsa2")
        .await?
        .deserialize_multiple()?;
    println!("{:?}", res);

    conn.execute("DROP TABLE test_dsadsa2").await?;
    return Ok(());
}
