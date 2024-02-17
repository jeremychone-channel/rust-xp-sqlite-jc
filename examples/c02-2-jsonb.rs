//! sqlite json doc: https://www.sqlite.org/json1.html

use rusqlite::Connection;
use serde_json::json;
use xp_sqlite::db_utils::{create_schema, print_select};
use xp_sqlite::Result;

fn main() -> Result<()> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;

	// -- Insert Row with json in data_t
	let json_data = json!({
		"prop_obj": {
			"name": "Obj 01",
			"is_big": true,
		},
		"prop_num": 111
	});
	let mut stmt = conn.prepare("INSERT INTO agent (name, level, data_b) VALUES (?1, ?2, jsonb(?3)) RETURNING id")?;
	let id = stmt.query_row(("c02-agent name", 3, json_data.to_string()), |r| r.get::<_, i64>(0))?;

	// -- Update json sub property
	conn.execute(
		r#"UPDATE agent SET data_b = 
						jsonb_set(data_b, 
							'$.prop_num', ?2, 
							'$.prop_obj.name', ?3, 
							'$.prop_obj.is_big', json(?4),
							'$.new_prop_ob.another.name', ?5
						) 
						WHERE id = ?1"#,
		(&id, &222, &"Obj 01 Updated", &"true", &"another name"),
	)?;

	print_select(&conn, "SELECT id, name, json(data_b) FROM agent")?;

	Ok(())
}
