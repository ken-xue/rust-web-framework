use std::collections::HashMap;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::deserialize::QueryableByName;
use diesel::sql_types::*;

use chrono::NaiveDateTime;
use serde_derive::Serialize;

pub fn establish_connection(url: String) -> MysqlConnection {
    MysqlConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url.clone()))
}

#[derive(Debug, QueryableByName, Serialize)]
pub struct TableInfo {
    #[sql_type = "Text"]
    table_name: String,
    #[sql_type = "Text"]
    entity_name: String,
    #[sql_type = "Text"]
    engine: String,
    #[sql_type = "Nullable<Text>"]
    table_comment: Option<String>,
    #[sql_type = "Timestamp"]
    create_time: NaiveDateTime,
}

pub fn query_table_info(connection: &mut MysqlConnection, table_name: &str) -> Option<TableInfo> {
    //实体名称首字母大写,下划线后一个字母转为首字母大写
    let result = diesel::sql_query("SELECT table_name,
    CONCAT(
        UPPER(SUBSTRING(SUBSTRING_INDEX(table_name, '_', 1), 1, 1)),
        SUBSTRING(SUBSTRING_INDEX(table_name, '_', 1), 2),
        UPPER(SUBSTRING(table_name, LOCATE('_', table_name) + 1, 1)),
        SUBSTRING(table_name, LOCATE('_', table_name) + 2)
    ) AS entity_name,
    engine, table_comment,
    CAST(create_time AS DATETIME) as create_time
    FROM information_schema.tables WHERE table_schema = (SELECT DATABASE()) AND table_name = ?")
        .bind::<Text, _>(table_name)
        .get_result::<TableInfo>(connection)
        .optional()
        .unwrap();
    result
}

#[derive(Debug, QueryableByName, Serialize)]
pub struct ColumnInfo {
    #[sql_type = "Text"]
    pub column_name: String,
    #[sql_type = "Text"]
    pub column_type: String,
    //映射成rust的类型
    #[sql_type = "Text"]
    pub column_mapping_type: String,
    #[sql_type = "Text"]
    pub column_comment: String,
    #[sql_type = "Text"]
    pub column_key: String,
    #[sql_type = "Text"]
    pub extra: String,
}

pub fn query_table_colum(connection: &mut MysqlConnection, table_name: &str) -> Vec<ColumnInfo> {
    let mut results = diesel::sql_query("select column_name column_name,column_name column_mapping_type, data_type column_type, column_comment column_comment, column_key column_key, extra from information_schema.columns where table_name = ? and table_schema = (select database()) order by ordinal_position")
        .bind::<Text, _>(table_name.to_string())
        .load::<ColumnInfo>(connection)
        .unwrap();
    //通过result.column_type作为map的key去TYPE_MAP中查找对应的value，如果没找到则使用result.column_type赋值给column_mapping_type
    for mut result in results.iter_mut() {
        if let Some(column_mapping_type) = TYPE_MAP.get(result.column_type.as_str()) {
            result.column_mapping_type = column_mapping_type.to_string();
        }
    }
    results
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub table_info: TableInfo,
    pub table_columns: Vec<ColumnInfo>,
}

pub fn get_table_info(conn: &mut MysqlConnection, table_name: &str) -> Table {
    let table_info = query_table_info(conn, table_name);
    let table_columns = query_table_colum(conn, table_name);
    return Table {
        table_info: table_info.unwrap(),
        table_columns,
    };
}

use lazy_static::lazy_static;

lazy_static! {
    static ref TYPE_MAP: HashMap<&'static str, &'static str> = {
    let mut map = HashMap::new();
    map.insert("tinyint","i32");
    map.insert("smallint","i32");
    map.insert("mediumint","i32");
    map.insert("int","i32");
    map.insert("integer","i32");
    map.insert("bigint","i64");
    map.insert("float","f32");
    map.insert("double","f64");
    map.insert("decimal","BigDecimal");
    map.insert("bit","bool");
    map.insert("char","String");
    map.insert("varchar","String");
    map.insert("tinytext","String");
    map.insert("text","String");
    map.insert("mediumtext","String");
    map.insert("longtext","String");
    map.insert("date","chrono::NaiveDate");
    map.insert("datetime","chrono::NaiveDateTime");
    map.insert("timestamp","chrono::NaiveDateTime");
    map.insert("NUMBER","i32");
    map.insert("INT","i32");
    map.insert("INTEGER","i32");
    map.insert("BINARY_INTEGER","i32");
    map.insert("LONG","String");
    map.insert("FLOAT","f32");
    map.insert("BINARY_FLOAT","f32");
    map.insert("DOUBLE","f64");
    map.insert("BINARY_DOUBLE","f64");
    map.insert("DECIMAL","BigDecimal");
    map.insert("CHAR","String");
    map.insert("VARCHAR","String");
    map.insert("VARCHAR2","String");
    map.insert("NVARCHAR","String");
    map.insert("NVARCHAR2","String");
    map.insert("CLOB","String");
    map.insert("BLOB","String");
    map.insert("DATE","chrono::NaiveDate");
    map.insert("DATETIME","chrono::NaiveDateTime");
    map.insert("TIMESTAMP","chrono::NaiveDateTime");
    map.insert("TIMESTAMP(6)","chrono::NaiveDateTime");
    map.insert("int8","i64");
    map.insert("int4","i32");
    map.insert("int2","i32");
    map.insert("numeric","BigDecimal");
    map.insert("nvarchar","String");
    map
};
}