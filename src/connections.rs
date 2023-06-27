use crate::{
    request::{post, post_raw},
    structs::{ExecuteRequest, ExecuteResponse, Session},
    utils::to_base64,
};
use anyhow::Result;
use async_mutex::Mutex;
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct PSConnection {
    pub(crate) host: String,
    pub(crate) auth: String,
    pub session: Arc<Mutex<Option<Session>>>,
    pub client: reqwest::Client,
}

impl PSConnection {
    /// Creates a new connection
    pub fn new(host: &str, username: &str, password: &str) -> Self {
        Self {
            host: host.into(),
            auth: format!("Basic {}", to_base64(&format!("{}:{}", username, password))),
            session: Arc::new(Mutex::new(None)),
            client: reqwest::Client::new(),
        }
    }

    /// Creates a new connection from the environment variables (DATABASE_HOST, DATABASE_USERNAME, DATABASE_PASSWORD)
    pub fn new_from_env() -> Result<Self> {
        Ok(PSConnection::new(
            &env::var("DATABASE_HOST")?,
            &env::var("DATABASE_USERNAME")?,
            &env::var("DATABASE_PASSWORD")?,
        ))
    }

    /// Execute a SQL query
    pub async fn execute(&self, query: &str) -> Result<()> {
        self.execute_raw(query).await?;
        Ok(())
    }

    /// Execute a SQL query and return the raw response
    pub async fn execute_raw(&self, query: &str) -> Result<ExecuteResponse> {
        let url = format!("https://{}/psdb.v1alpha1.Database/Execute", self.host);
        let sql = ExecuteRequest {
            query: query.into(),
            session: self.session.lock().await.clone(),
        };

        let res: ExecuteResponse = post(self, &url, sql).await?;
        self.session.lock().await.replace(res.session.clone());

        if let Some(err) = res.error {
            anyhow::bail!("Code: \"{}\", message: \"{}\"", err.code, err.message);
        }

        Ok(res)
    }

    /// As the name suggests, this function is making a transaction
    pub async fn transaction<F, Fut>(&self, f: F) -> Result<()>
    where
        F: FnOnce(Self) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        self.execute("BEGIN").await?;
        let res = f(self.clone()).await;
        if res.is_err() {
            self.execute("ROLLBACK").await?;
            return res;
        }

        self.execute("COMMIT").await?;
        Ok(())
    }

    /// Refreshes the session
    pub async fn refresh(&self) -> Result<()> {
        let url = format!("https://{}/psdb.v1alpha1.Database/CreateSession", self.host);
        let res: ExecuteResponse = post_raw(self, &url, String::from("{}")).await?;
        self.session.lock().await.replace(res.session.clone());

        Ok(())
    }
}
