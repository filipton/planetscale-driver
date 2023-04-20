use crate::{
    request::{post, post_raw},
    structs::{ExecuteRequest, ExecuteResponse, Session},
    utils::to_base64,
    QueryBuilder,
};
use anyhow::Result;

#[derive(Clone)]
pub struct PSConnection {
    pub(crate) host: String,
    pub(crate) auth: String,
    pub session: Option<Session>,
    pub client: reqwest::Client,
}

impl PSConnection {
    /// Creates a new connection
    pub fn new(host: &str, username: &str, password: &str) -> Self {
        Self {
            host: host.into(),
            auth: format!("Basic {}", to_base64(&format!("{}:{}", username, password))),
            session: None,
            client: reqwest::Client::new(),
        }
    }

    /// Executes a SQL query
    pub async fn execute(&mut self, query: &str) -> Result<ExecuteResponse> {
        let url = format!("https://{}/psdb.v1alpha1.Database/Execute", self.host);
        let sql = ExecuteRequest {
            query: query.into(),
            session: self.session.clone(),
        };

        let res: ExecuteResponse = post(self, &url, sql).await?;
        self.session = Some(res.session.clone());

        Ok(res)
    }

    pub fn query<T>(&mut self, query: &str) -> QueryBuilder {
        QueryBuilder::new(query)
    }

    /// Execute a multiple SQL queries using transactions
    pub async fn transaction(&self, q: Vec<QueryBuilder>) -> Result<()> {
        let mut conn = self.clone();
        conn.execute("BEGIN").await?;

        for query in q {
            let res = query.execute(&mut conn).await?;
            if let Some(err) = res.error {
                conn.execute("ROLLBACK").await?;
                anyhow::bail!("Code: \"{}\", message: \"{}\"", err.code, err.message);
            }
        }

        conn.execute("COMMIT").await?;
        return Ok(());
    }

    /// Refreshes the session
    pub async fn refresh(&mut self) -> Result<()> {
        let url = format!("https://{}/psdb.v1alpha1.Database/CreateSession", self.host);
        let res: ExecuteResponse = post_raw(self, &url, String::from("{}")).await?;
        self.session = Some(res.session.clone());

        Ok(())
    }
}
