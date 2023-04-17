use core::fmt;

use crate::{structs::ExecuteResponse, PSConnection};
use anyhow::Result;

pub struct QueryBuilder {
    pub query: String,
    pub values: Vec<String>,
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

    pub fn generated_sql(&self) -> String {
        let mut query = self.query.clone();
        for i in 0..self.values.len() {
            query = query.replace(&format!("${}", i), &self.values[i]);
        }

        query
    }

    pub async fn execute(self, connection: &PSConnection) -> Result<ExecuteResponse> {
        let mut query = self.query;
        for i in 0..self.values.len() {
            query = query.replace(&format!("${}", i), &self.values[i]);
        }

        connection.execute(&query).await
    }

    pub async fn execute_session(self, connection: &mut PSConnection) -> Result<ExecuteResponse> {
        let mut query = self.query;
        for i in 0..self.values.len() {
            query = query.replace(&format!("${}", i), &self.values[i]);
        }

        connection.execute_session(&query).await
    }
}

impl fmt::Debug for QueryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generated sql: {}", self.generated_sql())
    }
}
