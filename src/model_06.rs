use modql::field::{Fields, SeaFieldValue};
use modql::{SqliteFromRow, SqliteFromValue};

#[derive(Debug, SeaFieldValue, SqliteFromValue)]
pub enum Model {
	Gpt3,
	Gpt4,
}

#[derive(Debug, Fields, SqliteFromRow)]
pub struct Agent {
	pub id: i32,
	pub name: String,
	pub model: Option<Model>,
	pub level: Option<i64>,
	pub data_t: Option<serde_json::Value>,
	pub data_b: Option<Vec<u8>>,
}

#[derive(Debug, Fields, Default)]
pub struct AgentForCreate {
	pub name: String,
	pub level: Option<i64>,
	pub data_t: Option<serde_json::Value>,
	pub data_b: Option<Vec<u8>>,
}
