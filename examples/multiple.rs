use anyhow::Result;
use planetscale_driver::{query, Database, PSConnection};
use std::env::var;

#[derive(Database, Debug)]
pub struct TestDsadsa {
    pub id: u32,
    pub value: u32,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);

    query("CREATE TABLE IF NOT EXISTS test_dsadsa(id INT AUTO_INCREMENT PRIMARY KEY, value INT NOT NULL)")
        .execute(&mut conn)
        .await?;

    query("INSERT INTO test_dsadsa(value) VALUES (69), (420), (1337), (69420), (1234), (1111)")
        .execute(&mut conn)
        .await?;

    let res: Vec<TestDsadsa> = query("SELECT * FROM test_dsadsa")
        .fetch_all(&mut conn)
        .await?;
    println!("{:?}", res);

    conn.execute("DROP TABLE test_dsadsa").await?;
    return Ok(());
}
