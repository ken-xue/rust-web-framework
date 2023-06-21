// use std::fmt::{Error};
use std::ops::{DerefMut};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::util;
use crate::database;
use crate::system::models::SysUser;
use crate::database::schema::sys_user::dsl::*;
use crate::system::user_handler::{CreateUser, UpdateUser};
use crate::database::schema::sys_user;

pub struct UserRepo {
    conn: database::PoolConnection
}

impl UserRepo {

    pub fn new(conn: database::PoolConnection) -> Self {
        UserRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysUser, Error> {
        sys_user.filter(id.eq(i))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())
    }

    pub fn update(&mut self,user: SysUser) -> Result<SysUser, Error> {
        diesel::update(sys_user.filter(id.eq(user.id)))
            .set(&user)
            .execute(self.conn.deref_mut())
            .map_err(|e| {
                println!("Error updating user: {}", e);
            }).ok();
        Ok(user)
    }

    pub fn create(&mut self, user: SysUser) -> Result<SysUser, Error> {
        match diesel::insert_into(sys_user::table).values(&user).execute(self.conn.deref_mut()) {
            Ok(_) => Ok(user),
            Err(e) => Err(e)
        }
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, Error> {
        diesel::delete(sys_user.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional()
    }
}