pub mod database;

use crate::config;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::ops::Deref;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// 初始化连接池
// pub fn initialize(cfg: config::Config) {
//     let url = cfg.database.url;
    // let manager = ConnectionManager::<MysqlConnection>::new(url);
    // build pool
    // let pool = Pool::builder().build(manager);

    // let manager = ConnectionManager::<MysqlConnection>::new(url);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool");
// }

// 获取连接
// pub fn get_connection() -> MysqlConnection {
//     let pool = init_pool();
//     let conn = pool.get().unwrap();
//     conn
// }


use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Mutex;
use once_cell::sync::Lazy;

//
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// 全局连接池
static CONNECTION_POOL: Lazy<Mutex<Option<DbPool>>> = Lazy::new(|| Mutex::new(None));

// 初始化连接池
pub fn initialize(cfg: config::Config) {
    let url = &cfg.database.url;
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool.");

    let mut connection_pool = CONNECTION_POOL.lock().unwrap();
    *connection_pool = Some(pool);
}

// 获取连接
// pub fn get_connection() -> MysqlConnection {
//     let connection_pool = CONNECTION_POOL.lock().unwrap();
//     let pool = connection_pool
//         .as_ref()
//         .expect("Connection pool has not been initialized.");
//     pool.get_conn().expect("Failed to get connection from pool")
// }

pub fn get_connection() -> &'static MysqlConnection {
    let connection_pool = CONNECTION_POOL.lock().unwrap();
    let pooled_connection = connection_pool
        .as_ref()
        .expect("Connection pool has not been initialized.")
        .get()
        .expect("Failed to get connection from pool");
    pooled_connection.deref()
}