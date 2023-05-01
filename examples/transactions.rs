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

    query("CREATE TABLE test_dsadsa2(id INT AUTO_INCREMENT PRIMARY KEY, value INT NOT NULL)")
        .execute(&mut conn)
        .await?;

    conn.execute("INSERT INTO test_dsadsa2(value) VALUES (321), (654)")
        .await?;

    // Intentionally wrong query without catching the error
    _ = conn
        .transaction(|conn| async move {
            let mut conn = conn.lock().await;

            // good
            conn.execute("INSERT INTO test_dsadsa2(value) VALUES (69), (420), (1337), (69420), (1234), (1111)")
                .await?;

            // bad (beacuse of the wrong column name)
            conn.execute("INSERT INTO test_dsadsa2(valueccxzcxzcxz) VALUES (69), (420), (1337), (69420), (1234), (1111)")
                .await?;

            Ok(())
        })
        .await;

    let res: Vec<TestDsadsa> = query("SELECT * FROM test_dsadsa2")
        .fetch_all(&mut conn)
        .await?;
    println!("{:?}", res);

    conn.execute("DROP TABLE test_dsadsa2").await?;
    return Ok(());
}
