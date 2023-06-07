use std::fmt::{Debug, Error};
use diesel::RunQueryDsl;
use crate::database;
use crate::schema::posts;
use crate::schema::sys_user::dsl::sys_user;
use crate::schema::sys_user::name;

pub struct User {
    conn: database::PC,
}

impl User {

    pub fn new(conn: database::PC) -> Self {
        User { conn }
    }

    pub fn get_user_by_id(&self, id: i32) -> Result<User, Error> {
        use crate::schema::sys_user;
        let results = sys_user
            .filter(name.eq(true))
            .limit(5)
            .select(SysUser::as_select())
            .load(connection)
            .expect("Error loading posts");

    }


    pub fn create_user(&self, id: i32) -> Result<T, E> {
        use crate::schema::sys_user;
        diesel::insert_into(sys_user::table)
            .values(&new_post)
            .execute(conn).fmt()
    }

}