use planetscale_driver::{QueryBuilder, PSConnection};

#[tokio::main]
async fn main() {
    let conn = PSConnection::new(
        "aws.connect.psdb.cloud",
        "zrhq79gia2vqhporjydc",
        "pscale_pw_N11vup13sipUzd2cc8sY0nYxRp7WA0lEVfRydcizdwI",
    );

    let random_id = rand::random::<i32>();
    let random_count = rand::random::<i32>();
    let random_elon = rand::random::<f64>();
    let random_test = rand::random::<i32>();

    let q1 =
        QueryBuilder::new("INSERT INTO counter(id, count, elon, test) VALUES ($0, $1, $2, \"$3\")")
            .bind(random_id)
            .bind(random_count)
            .bind(random_elon)
            .bind(random_test);

    let q2 = QueryBuilder::new("SELECT 1'");

    let res = conn.transaction(vec![q1, q2]).await;
    println!("{:?}", res);
}
