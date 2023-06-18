use std::fmt::{Error};
use std::ops::{DerefMut};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::util;
use crate::database;
use crate::system::sys_model::SysUser;
use crate::schema::sys_user::dsl::*;
use crate::system::user_handler::CreateUser;

pub struct UserRepo {
    conn: database::PoolConnection,
}

impl UserRepo {

    pub fn new(conn: database::PoolConnection) -> Self {
        UserRepo { conn }
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
        let mut connection = database::pool(); // Get a connection from the pool
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

    pub fn create_user(&mut self, u: CreateUser) -> Result<SysUser, Error> {
        let user: SysUser = u.into();
        use crate::schema::sys_user;
        diesel::insert_into(sys_user::table)
            .values(&user)
            .execute(self.conn.deref_mut()).expect("Error while saving user");
        Ok(user)
    }
}

impl From<CreateUser> for SysUser {
    fn from(user: CreateUser) -> SysUser {
        SysUser {
            id: 0,
            uuid: Some(util::uuid()),
            account: Option::from(user.account),
            password: Option::from(user.password),
            name: Some(user.name),
            email: Option::from(user.email),
            status: None,
            creator: None,
            modifier: None,
            gmt_create: Default::default(),
            gmt_modified: Default::default(),
            avatar: None,
            deleted: false,
        }
    }
}