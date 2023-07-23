use std::ops::DerefMut;
use diesel::{BoxableExpression, ExpressionMethods, IntoSql, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use diesel::mysql::Mysql;
use diesel::result::Error;
use diesel::sql_types::Bool;

use crate::database;
use crate::system::dept::model::SysDept;
use crate::database::schema::sys_dept::dsl::*;
use crate::database::schema::sys_dept;
use crate::system::dept::request::{ListDept, PageDept};

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

    pub fn add(&mut self, d: SysDept) -> Result<SysDept, anyhow::Error> {
        diesel::insert_into(sys_dept::table).values(&d).execute(self.conn.deref_mut())?;
        Ok(d)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_dept.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: PageDept) -> Result<(Vec<SysDept>, i64), anyhow::Error> {
        let offset = page.page_size * (page.page - 1);
        let query_result = sys_dept.select(SysDept::as_select()).limit(page.page_size).offset(offset).load::<SysDept>(self.conn.deref_mut())?;
        let total_count = sys_dept.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysDept> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }

    pub fn list(&mut self,r: ListDept) -> Result<Vec<SysDept>, anyhow::Error> {
        let list = sys_dept.select(SysDept::as_select()).filter(condition_like_name(r.name)).load::<SysDept>(self.conn.deref_mut())?;
        Ok(list)
    }
}

fn condition_like_name(opt: Option<String>) -> Box<dyn BoxableExpression<sys_dept::table, Mysql, SqlType = Bool>> {
    match opt {
        Some(value) => Box::new(name.like(format!("%{}%", value))),
        None => Box::new(true.into_sql::<Bool>()) // 不加条件
    }
}