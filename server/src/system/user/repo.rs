use std::ops::{DerefMut};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::database;
use crate::system::user::model::SysUser;
use crate::database::schema::sys_user::dsl::*;

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

    pub fn get_by_username(&mut self, username: &str) -> Result<SysUser, Error> {
        sys_user.filter(username.eq(username))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())
    }

    pub fn update(&mut self,user: SysUser) -> Result<Option<usize>, Error> {
        diesel::update(sys_user.filter(id.eq(user.id)))
            .set(&user)
            .execute(self.conn.deref_mut()).optional()
    }

    pub fn create(&mut self, user: SysUser) -> Result<SysUser, Error> {
        match diesel::insert_into(sys_user::table).values(&user).execute(self.conn.deref_mut()) {
            Ok(_) => Ok(user),
            Err(e) => Err(e)
        }
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, Error> {
        diesel::delete(sys_user.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)
    }

    pub fn page(&mut self, page: i64, size: i64) -> Result<(Vec<SysUser>, i64), Error> {
        let offset = size * (page - 1);
        let query_result = sys_user.select(SysUser::as_select()).limit(size).offset(offset).load::<SysUser>(self.conn.deref_mut())?;
        let total_count = sys_user.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysUser> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }
}