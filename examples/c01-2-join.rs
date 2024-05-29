use rusqlite::Connection;
use xp_sqlite::db_utils::{create_schema, print_rows};
use xp_sqlite::Result;

fn main() -> Result<()> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;

	// -- Create a module
	conn.execute("INSERT INTO module (name) VALUES (?1)", ("Module-One",))?;
	let module_id = 1; // assume 1 for now. TODO add returning id above

	// -- Insert 5 Rows
	for i in 1..=5 {
		let name = format!("buddy-{:02}", i);
		let level = 10 + i;
		let data_t: Option<String> = None;
		conn.execute(
			"INSERT INTO agent (name, model, level, module_id, data_t) VALUES (?1, ?2, ?3, ?4, ?5)",
			(&name, "Gpt4", &level, &module_id, &data_t),
		)?;
	}

	// -- Insert `string` in `number` column
	// NOTE: FAIL with `STRICT`
	// conn.execute("INSERT INTO agent (name, level) VALUES (?1, ?2)", ("b", "no-a-number"))?;

	let query = "
    SELECT 
        agent.id,
        agent.name,
        agent.model,
				agent.module_id,
        module.name as module_name,
        agent.data_t,
        agent.data_b
    FROM 
        agent
    INNER JOIN 
        module ON agent.module_id = module.id
";

	// -- Simple query and print
	let mut stmt = conn.prepare(query)?;
	let rows = stmt.query([])?;
	println!("\n=== query (print_rows)\n");
	print_rows(rows)?;

	Ok(())
}
