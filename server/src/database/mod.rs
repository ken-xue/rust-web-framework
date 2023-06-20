pub mod database;
pub mod schema;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use r2d2::PooledConnection;
use std::env;
use crate::config;

pub type PoolConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// 创建一个全局的连接池
lazy_static::lazy_static! {
    static ref POOL: Pool<ConnectionManager<MysqlConnection>> = {
        let config = config::get();
        let url = config.database.url.clone();
        let manager = ConnectionManager::<MysqlConnection>::new(url);
        Pool::builder().max_size(10).build(manager).unwrap()
    };
}

// 获取连接池
pub fn pool() -> PoolConnection {
    let pool_connection = POOL.get().unwrap();
    return pool_connection
}

// 动态建立连接
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
