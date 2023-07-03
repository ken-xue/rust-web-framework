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

    pub fn default() -> Self {
        let conn = database::pool();
        UserRepo { conn }
    }

    pub fn new(conn: database::PoolConnection) -> Self {
        UserRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysUser, anyhow::Error> {
        let ret = sys_user
            .filter(id.eq(i))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn get_by_username(&mut self, _username: &str) -> Result<SysUser, anyhow::Error> {
        let ret = sys_user.filter(username.eq(_username))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn update(&mut self,user: SysUser) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::update(sys_user.filter(id.eq(user.id)))
            .set(&user)
            .execute(self.conn.deref_mut()).optional()?;
        Ok(ret)
    }

    pub fn create(&mut self, user: SysUser) -> Result<SysUser, anyhow::Error> {
        diesel::insert_into(sys_user::table).values(&user).execute(self.conn.deref_mut())?;
        Ok(user)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_user.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: i64, size: i64) -> Result<(Vec<SysUser>, i64), anyhow::Error> {
        let offset = size * (page - 1);
        let query_result = sys_user.select(SysUser::as_select()).limit(size).offset(offset).load::<SysUser>(self.conn.deref_mut())?;
        let total_count = sys_user.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysUser> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }
}