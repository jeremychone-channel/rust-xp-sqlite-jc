use rusqlite::Connection;
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

	// -- Insert `string` in `number` column
	// NOTE: FAIL with `STRICT`
	// conn.execute("INSERT INTO agent (name, level) VALUES (?1, ?2)", ("b", "no-a-number"))?;

	// -- Simple query and print
	let mut stmt = conn.prepare("SELECT * FROM agent WHERE level > :lvl")?;
	let rows = stmt.query(&[(":lvl", &12)])?;
	println!("\n=== query (print_rows)\n");
	print_rows(rows)?;

	// -- Print version
	print_select(&conn, "SELECT sqlite_version()")?;

	Ok(())
}
