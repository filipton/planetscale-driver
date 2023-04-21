use anyhow::Result;
use planetscale_driver::{query, Database, Deserializer, PSConnection, QueryBuilder};
use std::env::var;

#[derive(Database, Debug)]
pub struct TestDsadsa {
    pub id: u32,
    pub value: u32,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut conn = PSConnection::new(&var("PS_HOST")?, &var("PS_USER")?, &var("PS_PASS")?);
    let res = conn
        .trans(|c| async move {
            let mut d = c.lock().unwrap();

            d.execute_raw(
                "CREATE TABLE test_dsadsa(id INT AUTO_INCREMENT PRIMARY KEY, value INT NOT NULL)",
            )
            .await?;

            Ok(())
        })
        .await;

    println!("{:?}", res);

    /*
    query("CREATE TABLE test_dsadsa2(id INT AUTO_INCREMENT PRIMARY KEY, value INT NOT NULL)")
        .execute(&mut conn)
        .await?;

    conn.execute("INSERT INTO test_dsadsa2(value) VALUES (321), (654)")
        .await?;

    let q_correct = QueryBuilder::new(
        "INSERT INTO test_dsadsa2(value) VALUES (69), (420), (1337), (69420), (1234), (1111)",
    );
    let q_wrong = QueryBuilder::new(
        "INSERT INTO test_dsadsa2(valueccxzcxzcxz) VALUES (69), (420), (1337), (69420), (1234), (1111)",
    );

    // Intentionally wrong query without catching the error
    _ = conn.transaction(vec![q_correct, q_wrong]).await;

    let res: Vec<TestDsadsa> = query("SELECT * FROM test_dsadsa2")
        .fetch_all(&mut conn)
        .await?;
    println!("{:?}", res);

    conn.execute("DROP TABLE test_dsadsa2").await?;
    */
    return Ok(());
}
