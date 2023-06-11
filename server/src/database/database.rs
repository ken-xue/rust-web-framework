// // 创建一个全局的连接池
// lazy_static::lazy_static! {
//     static ref POOL: Pool<ConnectionManager<PgConnection>> = {
//         let manager = ConnectionManager::<PgConnection>::new("postgres://localhost/my_db");
//         Pool::builder().max_size(10).build(manager)
//     };
// }
//
// // 在异步函数中获取连接并执行查询
// async fn query_users() {
//     let conn = POOL.get().unwrap();   // 从池中获取连接
//     let results = conn.prepare_raw("SELECT * FROM users").unwrap();
//     // 使用 results 执行查询...
//     POOL.put(conn);   // 查询结束后将连接放回池中
// }
//
// // 在 Axum 中使用
// async fn handle_request(req: Request) -> impl Respond {
//     query_users().await;   // 异步执行 SQL 查询
//     // 返回响应...
// }