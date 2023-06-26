use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

#[derive(Queryable)]
struct TableInfo {
    table_name: String,
    engine: String,
    table_comment: Option<String>,
    create_time: String,
}

fn main() {
    let connection = MysqlConnection::establish("mysql://user:password@localhost/database").unwrap();
    let result = diesel::sql_query(
        "SELECT table_name tableName, engine, table_comment tableComment, create_time createTime \
         FROM information_schema.tables \
         WHERE table_schema = (SELECT DATABASE()) AND table_name = 'sys_user'",
    ).load::<TableInfo>(&connection).unwrap();
    println!("{:#?}", result);
}

fn query_table_info(table_name: &str) -> Vec<TableInfo> {
    let connection = MysqlConnection::establish("mysql://user:password@localhost/database").unwrap();
    let result = diesel::sql_query(
        "SELECT table_name tableName, engine, table_comment tableComment, create_time createTime \
         FROM information_schema.tables \
         WHERE table_schema = (SELECT DATABASE()) AND table_name = $1",
    ).bind::<diesel::sql_types::Text, _>(table_name).load::<TableInfo>(&connection).unwrap();
    result
}