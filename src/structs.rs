use serde::{Deserialize, Serialize};

// https://transform.tools/json-to-rust-serde

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRequest {
    pub query: String,
    pub session: Option<Session>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    pub session: Session,
    pub result: Option<QueryResult>,
    pub error: Option<VitessError>,
    pub timing: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VitessError {
    pub message: String,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub signature: String,
    pub vitess_session: VitessSession,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VitessSession {
    pub in_transaction: Option<bool>,
    pub autocommit: Option<bool>,
    pub shard_sessions: Option<Vec<ShardSession>>,
    pub options: Options,
    pub found_rows: Option<String>,
    pub row_count: Option<String>,
    #[serde(rename = "DDLStrategy")]
    pub ddlstrategy: String,
    #[serde(rename = "SessionUUID")]
    pub session_uuid: String,
    pub enable_system_settings: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShardSession {
    pub target: Target,
    pub transaction_id: String,
    pub tablet_alias: TabletAlias,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub keyspace: String,
    pub shard: String,
    pub tablet_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabletAlias {
    pub cell: String,
    pub uid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub included_fields: String,
    pub client_found_rows: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub rows_affected: Option<String>,
    pub insert_id: Option<String>,
    pub fields: Option<Vec<Field>>,
    pub rows: Option<Vec<Row>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub table: Option<String>,

    pub org_table: Option<String>,
    pub database: Option<String>,
    pub org_name: Option<String>,

    pub column_length: Option<i64>,
    pub charset: Option<i64>,
    pub flags: Option<i64>,
    pub column_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Row {
    pub lengths: Vec<String>,
    pub values: String,
}
