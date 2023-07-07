use std::ops::Deref;
// extern crate redis;
use redis::{Client, Commands, Connection, RedisResult};

// 创建一个全局的连接
lazy_static::lazy_static! {
    static ref CONN: Connection = {
        let client = redis::Client::open("redis://127.0.0.1/")
            .expect("Failed to create Redis client");
        let con = client.get_connection()
            .expect("Failed to get Redis connection");
        con
    };
}

// 集合某个value是否存在
pub fn exist(key:String, member: String) -> bool {
     match CONN.deref().sismember(key,member) {
         Ok(exist) => exist,
         Err(_) => false
     }
}
//添加集合
pub fn sadd(key:String, members: &[&str]) -> RedisResult<isize> {
    // 添加集合成员
    let size: i64 = CONN.deref().sadd(key,members)?;
    Ok(size as isize)
}


//
// fn fetch_an_integer() -> redis::RedisResult<isize> {
//     // connect to redis
//     let client = redis::Client::open("redis://127.0.0.1/")?;
//     let mut con = client.get_connection()?;
//     // throw away the result, just make sure it does not fail
//     let _ : () = con.set("my_key", 42)?;
//     // read back the key and return it.  Because the return value
//     // from the function is a result for integer this will automatically
//     // convert into one.
//     con.get("my_key")
// }
//
// fn ttttt() -> RedisResult<()> {
//     // 创建Redis客户端
//     let client = Client::open("redis://127.0.0.1/")?;
//     // 获取Redis连接
//     let mut con = client.get_connection()?;
//
//     // 添加集合成员
//     let _: i64 = con.sadd("fruits", &["apple", "banana", "orange"])?;
//
//     // 获取集合成员数量
//     let count: i64 = con.scard("fruits")?;
//     println!("fruits集合中有{}个成员", count);
//
//     // 判断集合中是否存在指定成员
//     let exists: bool = con.sismember("fruits", "apple")?;
//     println!("fruits集合中是否存在apple：{}", exists);
//
//     // 获取集合中的所有成员
//     let members: Vec<String> = con.smembers("fruits")?;
//     println!("fruits集合中的成员：{:?}", members);
//
//     // 从集合中移除成员
//     let _: i64 = con.srem("fruits", &["banana"])?;
//
//     // 获取集合中的所有成员
//     let members: Vec<String> = con.smembers("fruits")?;
//     println!("移除banana后，fruits集合中的成员：{:?}", members);
//
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     use redis::RedisError;
//     use super::*;
//     #[test]
//     fn rsas() {
//         // let i = fetch_an_integer();
//         // println!("size : {}",i)
//         // assert_eq!(&data[..], &dec_data[..]);
//     }
//     #[test]
//     fn rsasrrr() {
//         // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
//         // let mut con = client.get_connection().unwrap();
//         // let v:Result<String, RedisError> = conn.unwrap().sadd("mk", "dfa");
//         // println!("size : {:?}",v)
//         // assert_eq!(&data[..], &dec_data[..]);
//     }
//
//     #[test]
//     fn fetch_an_integer() {
//         let _ = ttttt();
//     }
// }