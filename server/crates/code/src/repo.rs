use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct TableInfo {
    table_name: String,
    engine: String,
    table_comment: Option<String>,
    create_time: NaiveDateTime,
}

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://root:123456@localhost/rwf";
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn query_table_info(table_name: &str) -> Vec<TableInfo> {
    let connection = &mut establish_connection();
    let results = diesel::sql_query("SELECT table_name, engine, table_comment, CAST(create_time AS DATETIME) as create_time FROM information_schema.tables WHERE table_schema = (SELECT DATABASE()) AND table_name = $1")
        .bind::<diesel::sql_types::Text, _>(table_name)
        .load(connection)
        .unwrap();
    results
}