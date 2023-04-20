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

    pub async fn execute(self, connection: &mut PSConnection) -> Result<ExecuteResponse> {
        let mut query = self.query;
        for i in 0..self.values.len() {
            query = query.replace(&format!("${}", i), &self.values[i]);
        }

        connection.execute(&query).await
    }

    pub async fn fetch_one<T>(self, connection: &PSConnection) -> Result<T>
    where
        T: Deserializer,
    {
        anyhow::bail!("TODO");
    }

    pub async fn fetch_all<T>(self, connection: &PSConnection) -> Result<Vec<T>>
    where
        T: Deserializer,
    {
        anyhow::bail!("TODO");
    }

    fn sql(&self) -> String {
        let mut query = self.query.clone();
        for i in 0..self.values.len() {
            query = query.replace(&format!("${}", i), &self.values[i]);
        }

        query
    }
}

impl fmt::Debug for QueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sql())
    }
}
