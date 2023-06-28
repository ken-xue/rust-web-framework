// // use diesel::MysqlConnection;
// //
// // //通过代码的方式导入diesel_cli依赖的方式进行类似于 `diesel print-schema -o sys_user_of_role` 的方式进行打印（注意不是直接调用cli）
// // pub fn create_schema(conn: &mut MysqlConnection,table_name: &str) {
// //     //TODO
// // }
//
// use diesel::mysql::MysqlConnection;
// use diesel::print_schema::schema_for_table;
//
// pub fn create_schema(conn: &MysqlConnection, table_name: &str) {
//     let table = schema_for_table(table_name);
//     println!("{}", diesel::debug_query(&table).to_string());
// }
