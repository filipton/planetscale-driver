use core::fmt;

use crate::{structs::ExecuteResponse, Deserializer, PSConnection};
use anyhow::Result;

pub struct QueryBuilder {
    query: String,
    values: Vec<String>,
}

impl QueryBuilder {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            values: Vec::new(),
        }
    }

    pub fn bind<T: ToString>(mut self, value: T) -> Self {
        let sanitized = value
            .to_string()
            .replace("'", "''")
            .replace("\"", "\\\"")
            .replace("`", "\\`");

        self.values.push(sanitized);
        self
    }

    fn generate_query(&self) -> String {
        let mut query = self.query.clone();
        for i in 0..self.values.len() {
            query = query.replace(&format!("${}", i), &self.values[i]);
        }

        query
    }

    pub async fn execute(self, connection: &mut PSConnection) -> Result<()> {
        connection.execute(&self.generate_query()).await
    }

    pub async fn execute_raw(self, connection: &mut PSConnection) -> Result<ExecuteResponse> {
        connection.execute_raw(&self.generate_query()).await
    }

    pub async fn fetch_one<T>(self, conn: &mut PSConnection) -> Result<T>
    where
        T: Deserializer,
    {
        let res = self.execute_raw(conn).await?;
        if let Some(err) = res.error {
            anyhow::bail!("Code: \"{}\", message: \"{}\"", err.code, err.message);
        }

        let res = res.deserialize()?;
        Ok(res)
    }

    pub async fn fetch_all<T>(self, conn: &mut PSConnection) -> Result<Vec<T>>
    where
        T: Deserializer,
    {
        let res = self.execute_raw(conn).await?;
        if let Some(err) = res.error {
            anyhow::bail!("Code: \"{}\", message: \"{}\"", err.code, err.message);
        }

        let res = res.deserialize_multiple()?;
        Ok(res)
    }
}

impl fmt::Debug for QueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generate_query())
    }
}
