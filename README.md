# PlanetScale Serverless Driver for Rust™

Rust "version" of [database-js](https://github.com/planetscale/database-js). As stated in database-js it uses PlanetScale HTTP api for database queries.
It will run perfectly run on Cloudflare Workers Or Vercel Edge Functions.

## Usage
NOTE: [Anyhow](https://crates.io/crates/anyhow) crate is required while using deserializer.

## Examples
### Connection and simple SQL execution
```rust
use planetscale_driver::PSConnection;

let conn = PSConnection::new(
  "<host>",
  "<user>",
  "<password>",
);
    
let res = conn.execute("SELECT 1").await.unwrap();
```

### Rows deserialization into struct
As you can see, deserialization doesn't use field names (MAYBE IN FUTURE) so remember to write your structs correctly!

```rust
use planetscale_driver::{Database, Deserializer};

#[derive(Database, Debug)]
struct TestD {
  val: u32
}

// ...

let res = conn.execute("SELECT 1").await.unwrap();
let res: TestD = res.deserialize().unwrap();

println!("{:?}", res);
```

### QueryBuilder
If you want to bind safely values into your query, you should use QueryBuilder

```rust
use planetscale_driver::QueryBuilder;

// ...

// note: values passed to .bind function must have trait ToString 
let id = 69;
let name = "420";

let res = QueryBuilder::new("INSERT INTO test(id, name) VALUES($0, \"$1\")")
  .bind(id)
  .bind(name)
  .execute(&conn)
  .await
  .unwrap();
```

### More examples in the [examples](examples) folder
If you want to run them:
```bash
PS_HOST=<host> PS_USER=<username> PS_PASS=<pscale_password> cargo run --example <example_name>
```
