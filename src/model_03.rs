use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Row, ToSql};

#[derive(Debug, strum::AsRefStr, strum::EnumString)]
pub enum Model {
	Gpt3,
	Gpt4,
}

// for rusqlite
impl FromSql for Model {
	fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
		let txt: String = FromSql::column_result(value)?;
		// Note: Because we have Model implementing FromStr with `strum::EnumString`
		let val: Model = txt.parse().map_err(|p_err| FromSqlError::Other(Box::new(p_err)))?;
		Ok(val)
	}
}

impl ToSql for Model {
	fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
		// `self.as_ref()` for `&str`, `into()` for rusqlite ValueRef
		Ok(ToSqlOutput::Borrowed(self.as_ref().into()))
	}
}

#[derive(Debug)]
pub struct Agent {
	pub id: i32,
	pub name: String,
	pub model: Option<Model>,
	pub level: Option<i64>,
	pub data_t: Option<serde_json::Value>,
	pub data_b: Option<Vec<u8>>,
}

#[derive(Debug, Default)]
pub struct AgentForCreate {
	pub name: String,
	pub model: Option<Model>,
	pub level: Option<i64>,
	pub data_t: Option<serde_json::Value>,
	pub data_b: Option<Vec<u8>>,
}

impl<'stmt> TryFrom<&Row<'stmt>> for Agent {
	type Error = rusqlite::Error;
	fn try_from(val: &Row<'stmt>) -> rusqlite::Result<Agent> {
		Ok(Self {
			id: val.get("id")?,
			name: val.get("name")?,
			model: val.get("model")?,
			level: val.get("level")?,
			data_t: val.get("data_t")?,
			data_b: val.get("data_b")?,
		})
	}
}
