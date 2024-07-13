use rusqlite::Connection;
use serde_json::json;
use xp_sqlite::db_utils::create_schema;
use xp_sqlite::model_03::{Agent, AgentForCreate, Model};
use xp_sqlite::Result;

fn main() -> Result<()> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;

	// -- Insert 5 Rows
	for i in 1..=5 {
		let data_t = json!({
			"name": "Some Object",
			"subObject": {
				"num": 123
			}
		});

		let agent_c = AgentForCreate {
			name: format!("buddy-{:02}", i),
			model: Some(Model::Gpt4),
			level: Some(10 + i),
			data_t: Some(data_t),
			..Default::default()
		};

		conn.execute(
			"INSERT INTO agent (name, model, level, data_t, data_b) VALUES (?1, ?2, ?3, ?4, ?5)",
			(
				&agent_c.name,
				&agent_c.model,
				&agent_c.level,
				&agent_c.data_t,
				&agent_c.data_b,
			),
		)?;
	}

	// -- With Query Map
	println!("\n=== query_map\n");
	let mut stmt = conn.prepare("SELECT id, name, model, level, data_t, data_b FROM agent WHERE level > :lvl")?;
	let agent_iter = stmt.query_map(&[(":lvl", &12)], |row| Agent::try_from(row))?;

	for agent in agent_iter {
		println!("{agent:?}");
	}

	Ok(())
}
