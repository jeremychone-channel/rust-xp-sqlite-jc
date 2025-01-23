use rusqlite::types::Value;
use rusqlite::{Connection, ToSql};
use xp_sqlite::db_utils::{create_schema, print_rows, print_select};
use xp_sqlite::Result;

fn main() -> Result<()> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;

	// -- Insert 5 Rows
	for i in 1..=5 {
		let name = format!("buddy-{:02}", i);
		let level = 10 + i;
		let data_t: Option<String> = None;
		conn.execute(
			"INSERT INTO agent (name, model, level, data_t) VALUES (?1, ?2, ?3, ?4)",
			(&name, "Gpt4", &level, &data_t),
		)?;
	}

	// -- Insert `number` in `string` column
	// OK in `strict` mode
	conn.execute("INSERT INTO agent (name, level) VALUES (?1, ?2)", (&123, &2000))?;

	// -- Example of dynamic column/values (from owned values, null as example)
	let null = Value::Null; // ok to just be a ref below
	let data: Vec<(String, Value)> = vec![("model".to_string(), null)];
	let (cols, vals): (Vec<String>, Vec<Value>) = data.iter().cloned().unzip();
	let cols_joined = cols.iter().map(|col| format!("\"{}\" = ?", col)).collect::<Vec<_>>().join(", ");
	let sql = format!("UPDATE agent SET {cols_joined}",);
	let dyn_vals: Vec<&dyn ToSql> = vals.iter().map(|x| x as &dyn ToSql).collect();
	conn.execute(&sql, &*dyn_vals)?;

	// -- Simple query and print
	let mut stmt = conn.prepare("SELECT agent.id, agent.name, agent.model FROM agent WHERE level > :lvl")?;
	let rows = stmt.query(&[(":lvl", &12)])?;
	println!("\n=== query (print_rows)\n");
	print_rows(rows)?;

	// -- Print version
	print_select(&conn, "SELECT sqlite_version()")?;

	Ok(())
}
