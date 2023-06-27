use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::deserialize::QueryableByName;
use diesel::sql_types::*;

use chrono::NaiveDateTime;

pub fn establish_connection(url: String) -> MysqlConnection {
    MysqlConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url.clone()))
}

#[derive(Debug, QueryableByName)]
pub struct TableInfo {
    #[sql_type = "Text"]
    table_name: String,
    #[sql_type = "Text"]
    engine: String,
    #[sql_type = "Nullable<Text>"]
    table_comment: Option<String>,
    #[sql_type = "Timestamp"]
    create_time: NaiveDateTime,
}

pub fn query_table_info(connection: & mut MysqlConnection, table_name: &str) -> Vec<TableInfo> {
    let results = diesel::sql_query("SELECT table_name, engine, table_comment, CAST(create_time AS DATETIME) as create_time FROM information_schema.tables WHERE table_schema = (SELECT DATABASE()) AND table_name = ?")
        .bind::<Text, _>(table_name)
        .load::<TableInfo>(connection)
        .unwrap();
    results
}

#[derive(Debug, QueryableByName)]
pub struct ColumnInfo {
    #[sql_type = "Text"]
    pub column_name: String,
    #[sql_type = "Text"]
    pub data_type: String,
    #[sql_type = "Text"]
    pub column_comment: String,
    #[sql_type = "Text"]
    pub column_key: String,
    #[sql_type = "Text"]
    pub extra: String,
}

pub fn query_table_colum(connection: & mut MysqlConnection, table_name: &str) -> Vec<ColumnInfo> {
    let results = diesel::sql_query(
        "select column_name,data_type, column_comment, column_key, extra from information_schema.columns where table_name = ? and table_schema = (select database()) order by ordinal_position")
        .bind::<Text, _>(table_name.to_string())
        .load::<ColumnInfo>(connection)
        .unwrap();
    results
}