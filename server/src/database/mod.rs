pub mod database;

use crate::config;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use r2d2::PooledConnection;
use std::env;
use std::sync::Mutex;

//
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type PC = PooledConnection<ConnectionManager<MysqlConnection>>;

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
pub fn get_connection() -> PC {
    let connection_pool = CONNECTION_POOL.lock().unwrap();
    let dp = connection_pool.as_ref().unwrap();
    let connection = dp.get().unwrap();
    return connection;
}

// 动态建立连接
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
