pub mod database;

use crate::config;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use r2d2::PooledConnection;
use std::env;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

pub type PoolConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

// 全局连接池
// static CONNECTION_POOL: Lazy<Mutex<Option<DbPool>>> = Lazy::new(|| Mutex::new(None));
pub static CONNECTION_POOL: Lazy<Arc<RwLock<Option<Pool<ConnectionManager<MysqlConnection>>>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));

// 初始化连接池
pub fn initialize(cfg: config::Config) {
    let url = &cfg.database.url;
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool.");
    let mut connection_pool = CONNECTION_POOL.write().unwrap();
    *connection_pool = Some(pool);
    println!("Connection pool initialized.");
}

// 获取连接池
pub fn pool() -> PoolConnection {
    initialize(config::initialize());//fix me,could not use global CONNECTION_POOL
    let connection_pool = CONNECTION_POOL.read().unwrap();
    let done = connection_pool.is_none();
    format!("is done {}",done);
    match connection_pool.as_ref() {
        Some(dp) => dp.get().unwrap(),
        None => panic!("Database connection pool is not initialized."),
    }
}

// 动态建立连接
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// 创建一个全局的连接池
lazy_static::lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<MysqlConnection>> = {
        let manager = ConnectionManager::<MysqlConnection>::new("mysql://root:123456@localhost/rust-web-framework?useUnicode=true&characterEncoding=UTF-8&useSSL=false&serverTimezone=UTC");
        Pool::builder().max_size(10).build(manager).unwrap()
    };
}
