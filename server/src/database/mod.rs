pub mod database;

use crate::config;
use diesel::prelude::*;
use dotenvy::dotenv;
// use r2d2_diesel::ConnectionManager;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// 初始化连接池
pub fn initialize(cfg: config::Config) {
    let url = cfg.database.url;
    // let manager = ConnectionManager::<MysqlConnection>::new(url);
    // build pool
    // let pool = Pool::builder().build(manager);

    // let manager = ConnectionManager::<MysqlConnection>::new(url);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool");
}

// 获取连接
pub fn get_connection() -> MysqlConnection {
    // let pool = init_pool();
    // let conn = pool.get().unwrap();
    // conn
}
