use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::deserialize::QueryableByName;
use serde_json::value::{self, Map, Value as Json};
use diesel::sql_types::*;

use chrono::NaiveDateTime;
use clap::builder::Str;

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

pub fn query_table_info(connection: &mut MysqlConnection, table_name: &str) -> Option<TableInfo> {
    let result = diesel::sql_query("SELECT table_name, engine, table_comment, CAST(create_time AS DATETIME) as create_time FROM information_schema.tables WHERE table_schema = (SELECT DATABASE()) AND table_name = ?")
        .bind::<Text, _>(table_name)
        .get_result::<TableInfo>(connection)
        .optional()
        .unwrap();
    result
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
    let results = diesel::sql_query("select column_name,data_type, column_comment, column_key, extra from information_schema.columns where table_name = ? and table_schema = (select database()) order by ordinal_position")
        .bind::<Text, _>(table_name.to_string())
        .load::<ColumnInfo>(connection)
        .unwrap();
    results
}

#[derive(Debug)]
pub struct Table {
    pub table_info : TableInfo,
    pub table_columns : Vec<ColumnInfo>,
}

pub fn get_table_info(conn: & mut MysqlConnection,table_name: &str) -> Table {
    let table_info = query_table_info(conn,table_name);
    let table_columns = query_table_colum(conn,table_name);
    return Table {
        table_info: table_info.unwrap(),
        table_columns,
    }
}