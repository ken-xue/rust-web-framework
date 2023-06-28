use std::collections::HashMap;
use std::fmt::format;
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
    #[diesel(sql_type = Text)]
    table_name: String,
    #[diesel(sql_type = Text)]
    engine: String,
    #[diesel(sql_type = Nullable<Text>)]
    table_comment: Option<String>,
    #[diesel(sql_type = Timestamp)]
    create_time: NaiveDateTime,
}

pub fn query_table_info(connection: &mut MysqlConnection, table_name: &str) -> Option<TableInfo> {
    //实体名称首字母大写,下划线后一个字母转为首字母大写
    let result = diesel::sql_query("SELECT table_name,engine,table_comment,CAST(create_time AS DATETIME) as create_time
    FROM information_schema.tables WHERE table_schema = (SELECT DATABASE()) AND table_name = ?")
        .bind::<Text, _>(table_name)
        .get_result::<TableInfo>(connection)
        .optional()
        .unwrap();
    result
}

#[derive(Debug, QueryableByName, Serialize)]
pub struct ColumnInfo {
    #[diesel(sql_type = Text)]
    pub column_name: String,
    #[diesel(sql_type = Text)]
    pub column_type: String,
    //映射成rust的类型
    #[diesel(sql_type = Text)]
    pub column_mapping_type: String,
    #[diesel(sql_type = Text)]
    pub column_comment: String,
    #[diesel(sql_type = Text)]
    pub column_key: String,
    #[diesel(sql_type = Text)]
    pub is_nullable: String,
    #[diesel(sql_type = Text)]
    pub extra: String,
}

pub fn query_table_colum(connection: &mut MysqlConnection, table_name: &str) -> Vec<ColumnInfo> {
    let mut results = diesel::sql_query("select column_name column_name,column_name column_mapping_type, data_type column_type, column_comment column_comment, column_key column_key,IS_NULLABLE is_nullable, extra from information_schema.columns where table_name = ? and table_schema = (select database()) order by ordinal_position")
        .bind::<Text, _>(table_name.to_string())
        .load::<ColumnInfo>(connection)
        .unwrap();
    //通过result.column_type作为map的key去TYPE_MAP中查找对应的value，如果没找到则使用result.column_type赋值给column_mapping_type
    for mut result in results.iter_mut() {
        if let Some(column_mapping_type) = TYPE_MAP.get(result.column_type.as_str()) {
            if result.is_nullable.eq("YES") {//适配diesel加上Option<>
                result.column_mapping_type = "Option<".to_owned()+&column_mapping_type.to_string()+">";
            } else {
                result.column_mapping_type = column_mapping_type.to_string();
            }
        }
    }
    results
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub table_info: TableInfo,
    pub remove_prefix_table_name: String,//table_name移除前缀的
    pub entity_name: String,
    pub module_name: String,
    pub remove_prefix_entity_name: String,//entity_name移除前缀的
    pub table_columns: Vec<ColumnInfo>,
}

pub fn get_table_info(conn: &mut MysqlConnection,module_name: String,table_name: &str, prefix: Option<Vec<String>>) -> Table {
    //获取表数据
    let opt_table_info = query_table_info(conn, table_name);
    let table_columns = query_table_colum(conn, table_name);
    let table_info = opt_table_info.unwrap();
    let mut remove_prefix_table_name = table_info.table_name.to_string();
    //对table_name取首字母大写且下划线去掉取首字母大写
    let entity_name = get_entity_name(table_info.table_name.to_string().as_str());
    let mut remove_prefix_entity_name = entity_name.to_string();
    // 遍历prefix，如果当前table_info的table_name有该前缀则进行移除
    if let Some(prefixes) = prefix {
        for prefix in prefixes {
            if remove_prefix_table_name.starts_with(&prefix) {
                remove_prefix_table_name = remove_prefix_table_name[prefix.len()..].to_owned();
                remove_prefix_entity_name = get_entity_name(remove_prefix_table_name.as_str());
                break;
            }
        }
    }
    // 返回构造好的数据
    return Table {
        table_info,
        remove_prefix_table_name,
        entity_name,
        module_name,
        remove_prefix_entity_name,
        table_columns,
    };
}

// 首字母大写且下划线去掉取首字母大写
pub fn get_entity_name(table_name: &str) -> String {
    table_name
        .split('_')
        .map(|part| {
            let first_letter = part.chars().next().unwrap().to_uppercase().to_string();
            let remaining_letters = part.chars().skip(1).collect::<String>();
            first_letter + &remaining_letters
        })
        .collect::<String>()
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