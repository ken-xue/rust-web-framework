use std::ops::DerefMut;
use diesel::{allow_tables_to_appear_in_same_query, ExpressionMethods, joinable, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use diesel::result::Error;

use crate::database;
use crate::system::role::model::{SysRole};
use crate::database::schema::sys_role::dsl::*;
use crate::database::schema::{sys_role};
use crate::database::schema::sys_user_of_role::dsl::sys_user_of_role;
use crate::database::schema::sys_user_of_role::user_uuid;
use crate::system::role::request::PageRole;

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

    pub fn add(&mut self, d: SysRole) -> Result<SysRole, anyhow::Error> {
        diesel::insert_into(sys_role::table).values(&d).execute(self.conn.deref_mut())?;
        Ok(d)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_role.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: PageRole) -> Result<(Vec<SysRole>, i64), anyhow::Error> {
        let offset = page.page_size * (page.page - 1);
        let query_result = sys_role.select(SysRole::as_select()).limit(page.page_size).offset(offset).load::<SysRole>(self.conn.deref_mut())?;
        let total_count = sys_role.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysRole> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }

    pub fn list(&mut self) -> Result<Vec<SysRole>, anyhow::Error> {
        let list = sys_role.select(SysRole::as_select()).load::<SysRole>(self.conn.deref_mut())?;
        Ok(list)
    }

    pub fn get_by_user_uuid(&mut self,uid: String) -> Result<Vec<SysRole>, anyhow::Error> {
        use crate::database::schema::sys_user_of_role::role_uuid;
        let data = sys_user_of_role
            .filter(user_uuid.eq(uid))
            .inner_join(sys_role.on(uuid.eq(role_uuid)))
            .select(SysRole::as_select())
            .load::<SysRole>(self.conn.deref_mut())?;
        Ok(data)
    }
}