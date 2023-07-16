use std::ops::DerefMut;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::database;
use crate::system::dept::model::SysDept;
use crate::database::schema::sys_dept::dsl::*;
use crate::database::schema::sys_dept;

pub struct DeptRepo {
    conn: database::PoolConnection
}

impl DeptRepo {

    pub fn default() -> Self {
        let conn = database::pool();
        DeptRepo { conn }
    }

    pub fn new(conn: database::PoolConnection) -> Self {
        DeptRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysDept, anyhow::Error> {
        let ret = sys_dept.filter(id.eq(i))
            .select(SysDept::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn update(&mut self,d: SysDept) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::update(sys_dept.filter(id.eq(d.id)))
            .set(&d)
            .execute(self.conn.deref_mut()).optional()?;
        Ok(ret)
    }

    pub fn create(&mut self, d: SysDept) -> Result<SysDept, anyhow::Error> {
        diesel::insert_into(sys_dept::table).values(&d).execute(self.conn.deref_mut())?;
        Ok(d)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_dept.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: i64, size: i64) -> Result<(Vec<SysDept>, i64), anyhow::Error> {
        let offset = size * (page - 1);
        let query_result = sys_dept.select(SysDept::as_select()).limit(size).offset(offset).load::<SysDept>(self.conn.deref_mut())?;
        let total_count = sys_dept.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysDept> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }

    pub fn list(&mut self) -> Result<Vec<SysDept>, anyhow::Error> {
        let list = sys_dept.select(SysDept::as_select()).load::<SysDept>(self.conn.deref_mut())?;
        Ok(list)
    }
}