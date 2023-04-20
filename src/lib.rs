pub use crate::connections::PSConnection;
pub use deserializer::Deserializer;
pub use ps_driver_deserializer::Database;
pub use querybuilder::QueryBuilder;

mod connections;
mod deserializer;
mod querybuilder;
mod request;
mod structs;
mod utils;

/// Creates a new query builder (wrapper around `QueryBuilder::new`)
pub fn query(query: &str) -> QueryBuilder {
    QueryBuilder::new(query)
}
