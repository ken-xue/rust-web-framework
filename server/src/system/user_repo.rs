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

    pub fn get_by_id(&mut self, i: u64) -> Result<Option<SysUser>, Error> {
        Ok(sys_user.filter(id.eq(i))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())
            .optional()?)
    }

    pub fn update(&mut self,u: UpdateUser) -> Result<SysUser, Error> {
        let user: SysUser = u.into();
        diesel::update(sys_user.filter(id.eq(user.id)))
            .set((name.eq("James"), account.eq("Bond")))
            .execute(self.conn.deref_mut())
            .map_err(|e| {
                println!("Error updating user: {}", e);
            }).ok();
        Ok(user)
    }

    pub fn create(&mut self, u: CreateUser) -> Result<SysUser, Error> {
        let user: SysUser = u.into();
        diesel::insert_into(sys_user::table).values(&user)
            .execute(self.conn.deref_mut()).expect("Error while saving user");
        Ok(user)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) {
        // let _ = diesel::delete(sys_user.filter(id.eq(1))).execute(self.conn.deref_mut());
        diesel::delete(sys_user.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut())
            .map_err(|e| {
                println!("Error batch deleting users: {}", e);
            })
            .ok();
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

impl From<UpdateUser> for SysUser {
    fn from(user: UpdateUser) -> SysUser {
        SysUser {
            id: user.id,
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


#[derive(Debug)]
enum UserRepoError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidUsername,
}