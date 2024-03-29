use std::ops::DerefMut;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::database;
use crate::system::config::model::SysConfig;
use crate::database::schema::sys_config::dsl::*;
use crate::database::schema::sys_config;
use crate::system::config::request::PageConfig;

pub struct ConfigRepo {
    conn: database::PoolConnection
}

impl ConfigRepo {

    pub fn default() -> Self {
        let conn = database::pool();
        ConfigRepo { conn }
    }

    pub fn new(conn: database::PoolConnection) -> Self {
        ConfigRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysConfig, anyhow::Error> {
        let ret = sys_config.filter(id.eq(i))
            .select(SysConfig::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn update(&mut self,d: SysConfig) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::update(sys_config.filter(id.eq(d.id)))
            .set(&d)
            .execute(self.conn.deref_mut()).optional()?;
        Ok(ret)
    }

    pub fn add(&mut self, d: SysConfig) -> Result<SysConfig, anyhow::Error> {
        diesel::insert_into(sys_config::table).values(&d).execute(self.conn.deref_mut())?;
        Ok(d)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_config.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: PageConfig) -> Result<(Vec<SysConfig>, i64), anyhow::Error> {
        let offset = page.page_size * (page.page - 1);
        let query_result = sys_config.select(SysConfig::as_select()).limit(page.page_size).offset(offset).load::<SysConfig>(self.conn.deref_mut())?;
        let total_count = sys_config.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysConfig> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }

    pub fn list(&mut self) -> Result<Vec<SysConfig>, anyhow::Error> {
        let list = sys_config.select(SysConfig::as_select()).load::<SysConfig>(self.conn.deref_mut())?;
        Ok(list)
    }
}