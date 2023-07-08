use std::ops::DerefMut;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::database;
use crate::system::role::model::SysRole;
use crate::database::schema::sys_role::dsl::*;
use crate::database::schema::sys_role;

pub struct RoleRepo {
    conn: database::PoolConnection
}

impl RoleRepo {

    pub fn default() -> Self {
        let conn = database::pool();
        RoleRepo { conn }
    }

    pub fn new(conn: database::PoolConnection) -> Self {
        RoleRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysRole, anyhow::Error> {
        let ret = sys_role.filter(id.eq(i))
            .select(SysRole::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn update(&mut self,d: SysRole) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::update(sys_role.filter(id.eq(d.id)))
            .set(&d)
            .execute(self.conn.deref_mut()).optional()?;
        Ok(ret)
    }

    pub fn create(&mut self, d: SysRole) -> Result<SysRole, anyhow::Error> {
        diesel::insert_into(sys_role::table).values(&d).execute(self.conn.deref_mut())?;
        Ok(d)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_role.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: i64, size: i64) -> Result<(Vec<SysRole>, i64), anyhow::Error> {
        let offset = size * (page - 1);
        let query_result = sys_role.select(SysRole::as_select()).limit(size).offset(offset).load::<SysRole>(self.conn.deref_mut())?;
        let total_count = sys_role.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysRole> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }
}