use std::fmt::Error;
use diesel::RunQueryDsl;
use crate::database;
use crate::schema::posts;

pub struct User {
    conn: database::PC,
}

impl User {

    pub fn new(conn: database::PC) -> Self {
        User { conn }
    }

    pub fn get_user_by_id(&self, id: i32) -> Result<User, Error> {
        // 使用 conn 执行查询并返回 User
        // diesel::insert_into(posts::table)
        //     .values(&new_post)
        //     .execute(conn)?;
        // Ok(User)
    }
}