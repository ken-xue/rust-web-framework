use std::fmt::{Error};
use std::ops::{Deref, DerefMut};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::row::NamedRow;
use tracing::error;
use crate::database;
use crate::models::Post;
use crate::schema::posts::dsl::posts;
use crate::schema::sys_user::dsl::sys_user;
use crate::schema::sys_user::{id, name};
use crate::system::sys_model::SysUser;

pub struct User {
    conn: database::PoolConnection
}

impl User {

    pub fn new(conn: database::PoolConnection) -> Self {
        User { conn }
    }

    pub fn get_user_by_id(&mut self, i: i32) -> Result<SysUser, Error> {
        use crate::schema::sys_user::dsl::*;
        let connection = self.conn.deref_mut();
        let user : SysUser = sys_user
            .filter(id.eq(i))
            .limit(1)
            .select(SysUser::as_select())
            .load(connection)
            .expect("Error loading user");
        // println!("Displaying {} posts", results.len());
        Ok(user)
    }


    pub fn create_user(&mut self) {//-> Result<T, E> {
        // use crate::schema::sys_user;
        // let new_user = SysUser{
        //     id: 0,
        //     uuid: None,
        //     account: None,
        //     password: None,
        //     name: Option::from(format!("afdsfa")),
        //     email: None,
        //     status: None,
        //     creator: None,
        //     modifier: None,
        //     gmt_create: (),
        //     gmt_modified: (),
        //     deleted: None,
        //     avatar: None,
        // };
        // self.conn.transaction(|conn| {
        //     diesel::insert_into(sys_user::table)
        //         .values(&new_user)
        //         .execute(conn)?;
        //
        //     sys_user::table
        //         // .order(SysUser::id.desc())
        //         .select(SysUser::as_select())
        //         .first(conn)
        // })
        //     .expect("Error while saving post")
        // diesel::insert_into(sys_user::table)
        //     .values(&new_user)
        //     .execute(self.conn.deref())
    }

}