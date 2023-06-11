use std::fmt::{Error};
use std::ops::{Deref, DerefMut};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::database;
use crate::system::sys_model::SysUser;
use crate::schema::sys_user::dsl::*;

pub struct User {
    conn: database::PoolConnection,
}

impl User {
    pub fn new(conn: database::PoolConnection) -> Self {
        User { conn }
    }

    pub fn get_user_by_id(&mut self, i: u64) -> Result<SysUser, Error> {
        Ok(sys_user
            .filter(id.eq(i))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())
            .expect("Error loading user"))
    }

    pub fn update_user(&mut self) {
        use diesel::prelude::*;
        use crate::models::Post;
        use crate::schema::posts::dsl::*;
        let mut connection = database::POOL.get().unwrap(); // Get a connection from the pool
        let conn = connection.deref_mut(); // Convert the connection to a mutable reference

        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .select(Post::as_select())
            .load::<Post>(conn) // Pass the mutable reference to the connection
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
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