use crate::{
    request::{post, post_raw},
    structs::{ExecuteRequest, ExecuteResponse, Session},
    utils::to_base64,
};
use anyhow::Result;
use async_mutex::Mutex;
use std::sync::Arc;

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

    /// Execute a SQL query
    pub async fn execute(&mut self, query: &str) -> Result<()> {
        self.execute_raw(query).await?;
        Ok(())
    }

    /// Execute a SQL query and return the raw response
    pub async fn execute_raw(&mut self, query: &str) -> Result<ExecuteResponse> {
        let url = format!("https://{}/psdb.v1alpha1.Database/Execute", self.host);
        let sql = ExecuteRequest {
            query: query.into(),
            session: self.session.clone(),
        };

        let res: ExecuteResponse = post(self, &url, sql).await?;
        self.session = Some(res.session.clone());

        if let Some(err) = res.error {
            anyhow::bail!("Code: \"{}\", message: \"{}\"", err.code, err.message);
        }

        Ok(res)
    }

    /// As the name suggests, this function is making a transaction
    pub async fn transaction<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce(Arc<Mutex<Self>>) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let cloned_conn = Arc::new(Mutex::new(self.clone()));

        cloned_conn.lock().await.execute("BEGIN").await?;
        let res = f(cloned_conn.clone()).await;
        if res.is_err() {
            cloned_conn.lock().await.execute("ROLLBACK").await?;
            return res;
        }

        cloned_conn.lock().await.execute("COMMIT").await?;
        Ok(())
    }

    /// Refreshes the session
    pub async fn refresh(&mut self) -> Result<()> {
        let url = format!("https://{}/psdb.v1alpha1.Database/CreateSession", self.host);
        let res: ExecuteResponse = post_raw(self, &url, String::from("{}")).await?;
        self.session = Some(res.session);

        Ok(())
    }
}
