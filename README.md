# PlanetScale Serverless Driver for Rust™

<div align="center">
  <!-- Version -->
  <a href="https://crates.io/crates/planetscale-driver">
    <img src="https://img.shields.io/crates/v/planetscale-driver.svg?style=flat-square"
        alt="Crates.io version" />
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/planetscale-driver">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
        alt="docs.rs docs" />
  </a>
</div>

Rust "version" of [database-js](https://github.com/planetscale/database-js). As stated in database-js it uses PlanetScale HTTP api for database queries.
It will run perfectly run on Cloudflare Workers Or Vercel Edge Functions.

## Usage
```bash
cargo add planetscale-driver
# also "cargo add anyhow"
```

Then proceed like in examples!

## Examples
### Connection and simple SQL execution
```rust
use planetscale_driver::PSConnection;

let mut conn = PSConnection::new(
  "<host>",
  "<user>",
  "<password>",
);
    
let res = conn.execute("SELECT 1").await.unwrap();
```

### fetch_one/fetch_all/fetch_scalar
As you can see, deserialization doesn't use field names (MAYBE IN FUTURE) so remember to write your structs correctly!

```rust
use planetscale_driver::{Database, query};

#[derive(Database, Debug)]
struct TestD {
  val: u32
}

// ...

let res: TestD = query("SELECT 1").fetch_one(&mut conn).await?;
println!("{:?}", res);

let res: Vec<TestD> = query("SELECT val FROM testds").fetch_all(&mut conn).await?;
println!("{:?}", res);

let res: bool = query("SELECT true").fetch_scalar(&mut conn).await?;
println!("{:?}", res);
```

### QueryBuilder
If you want to bind safely values into your query, you should use QueryBuilder

Note: now query method is wrapper around QueryBuilder

```rust
// ...

// note: values passed to .bind function must have trait ToString 
let id = 69;
let name = "420";

// res there will be empty result, if you want to get reponse data use "execute_raw"
let res = query("INSERT INTO test(id, name) VALUES($0, \"$1\")")
  .bind(id)
  .bind(name)
  .execute(&mut conn)
  .await?;
```

### Json
```rust
use planetscale_driver::{Database, DatabaseJSON, query};

#[derive(Database, Debug)]
pub struct TestD {
    pub val: u32,
    pub test: TestJSON,
}

#[derive(DatabaseJSON, serde::Deserialize, serde::Serialize, Debug)]
pub struct TestJSON {
    pub text: String,
    pub test: u32,
}

// ...

let json: TestJSON = TestJSON {
    text: "test1234321".to_string(),
    test: 1234,
};

let res: TestD = query("SELECT 1010, '$0'").bind(json).fetch_one(&mut conn).await?;
println!("{:?}", res);
```

### Transactions
```rust
// ...

// NOTE: conn in closure isn't affecting main conn, its copied so session
// isn't modifed on "original" conn
conn.transaction(|conn| async move {
    //             ^- conn is Arc Mutex so we must do that
    //                note: it's not normal mutex (it's async mutex)
    let mut conn = conn.lock().await;

    conn.execute("QUERY")
        .await?;

    conn.execute("OTHER QUERY")
        .await?;

    //     ^- use question mark to propagate errors "up"
    //        it's required if you want to rollback changes after error

    Ok(())
}).await?;
```

### Features

This crate uses [reqwest](https://docs.rs/reqwest/latest/reqwest/) for making http requests. By default, this crate will use the `default-tls` feature of `reqwest` which may not build in your environment (e.g. netlify serverless functions). You can override this by disabling the default features of this crate and enabling a different reqwest tls feature like so:

```toml
planetscale-driver = {version="0.3.2", default-features=false}
reqwest= {version= "0.11.17", default-features=false, features=["rustls-tls"]}
```

> **Warning**
> If you simply disable default features and do not enable a different tls feature, reqwest will panic at runtime.


### More examples in the [examples](examples) folder
If you want to run them:
```bash
PS_HOST=<host> PS_USER=<username> PS_PASS=<pscale_password> cargo run --example <example_name>
```
