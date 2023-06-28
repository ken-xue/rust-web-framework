use std::ops::DerefMut;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::database;
use crate::system::menu::model::SysMenu;
use crate::database::schema::sys_menu::dsl::*;
use crate::database::schema::sys_menu;

pub struct MenuRepo {
    conn: database::PoolConnection
}

impl MenuRepo {

    pub fn new(conn: database::PoolConnection) -> Self {
        MenuRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysMenu, Error> {
        sys_menu.filter(id.eq(i))
            .select(SysMenu::as_select())
            .first(self.conn.deref_mut())
    }

    pub fn update(&mut self,d: SysMenu) -> Result<Option<usize>, Error> {
        diesel::update(sys_menu.filter(id.eq(d.id)))
            .set(&d)
            .execute(self.conn.deref_mut()).optional()
    }

    pub fn create(&mut self, d: SysMenu) -> Result<SysMenu, Error> {
        match diesel::insert_into(sys_menu::table).values(&d).execute(self.conn.deref_mut()) {
            Ok(_) => Ok(d),
            Err(e) => Err(e)
        }
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, Error> {
        diesel::delete(sys_menu.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)
    }

    pub fn page(&mut self, page: i64, size: i64) -> Result<(Vec<SysMenu>, i64), Error> {
        let offset = size * (page - 1);
        let query_result = sys_menu.select(SysMenu::as_select()).limit(size).offset(offset).load::<SysMenu>(self.conn.deref_mut())?;
        let total_count = sys_menu.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysMenu> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }
}