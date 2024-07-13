use rusqlite::Connection;
use sea_query::{Iden, IntoIden, Order, Query, SimpleExpr, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;
use serde_json::json;
use xp_sqlite::db_utils::create_schema;
use xp_sqlite::model_03::{Agent, AgentForCreate};
use xp_sqlite::Result;

#[derive(Iden)]
enum AgentIden {
	#[iden = "agent"]
	Table,
	Id,
	Name,
	Model,
	Level,
	DataT,
	DataB,
}

fn main() -> Result<()> {
	// -- Memory SQLite
	let conn = Connection::open_in_memory()?;
	create_schema(&conn)?;

	for i in 1..=3 {
		let data_t = json!({
			"name": "Some Object",
			"subObject": {
				"num": 123
			}
		});
		let agent = AgentForCreate {
			name: format!("buddy-{:02}", i),
			data_t: Some(data_t),
			..Default::default()
		};

		let columns = vec![AgentIden::Name.into_iden(), AgentIden::DataT.into_iden()];
		let values = vec![SimpleExpr::Value(agent.name.into()), SimpleExpr::Value(agent.data_t.into())];

		let mut query = Query::insert();
		let query = query.into_table(AgentIden::Table).columns(columns).values(values)?;
		let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);

		conn.execute(&sql, &*values.as_params())?;
	}

	let columns = vec![
		AgentIden::Id.into_iden(),
		AgentIden::Name.into_iden(),
		AgentIden::Model.into_iden(),
		AgentIden::Level.into_iden(),
		AgentIden::DataT.into_iden(),
		AgentIden::DataB.into_iden(),
	];
	let mut query = Query::select();

	let query = query
		.from(AgentIden::Table)
		.columns(columns)
		.order_by(AgentIden::Id.into_iden(), Order::Asc);

	let (sql, _values) = query.build_rusqlite(SqliteQueryBuilder);

	println!("Select statement: {sql}");

	let mut stmt = conn.prepare(&sql)?;
	let agent_iter = stmt.query_map([], |row| Agent::try_from(row))?;

	println!("\nResult:\n");
	for agent in agent_iter {
		println!("{:?}", agent.unwrap());
	}
	Ok(())
}
