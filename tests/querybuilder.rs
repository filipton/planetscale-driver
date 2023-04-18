use planetscale_driver::QueryBuilder;

#[tokio::test]
pub async fn simple_query() {
    let query = QueryBuilder::new("SELECT * FROM test");

    assert_eq!(query.generated_sql(), "SELECT * FROM test");
}

#[tokio::test]
pub async fn query_simple_bind() {
    let query = QueryBuilder::new("SELECT * FROM test WHERE id = $0").bind(69);

    assert_eq!(query.generated_sql(), "SELECT * FROM test WHERE id = 69");
}

#[tokio::test]
pub async fn query_more_advanced_bind() {
    let query = QueryBuilder::new(
        "SELECT *, (SELECT COUNT(*) FROM test2 WHERE other_id = $0) FROM test WHERE id = $0",
    )
    .bind(69);

    assert_eq!(
        query.generated_sql(),
        "SELECT *, (SELECT COUNT(*) FROM test2 WHERE other_id = 69) FROM test WHERE id = 69"
    );
}

#[tokio::test]
pub async fn query_sql_injection_bind() {
    let query = QueryBuilder::new("SELECT * FROM test WHERE text = \"$0\"")
        .bind("dsa\"; DROP DATABASE; --");

    assert_eq!(
        query.generated_sql(),
        "SELECT * FROM test WHERE text = \"dsa\\\"; DROP DATABASE; --\""
    );
}
