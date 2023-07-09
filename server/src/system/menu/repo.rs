use std::ops::DerefMut;
use diesel::{ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;
use diesel_logger::LoggingConnection;

use crate::database;
use crate::system::menu::model::SysMenu;
use crate::database::schema::sys_menu::dsl::*;
use crate::database::schema::sys_menu;
use crate::database::schema::sys_role_of_menu::dsl::sys_role_of_menu;
use crate::database::schema::sys_role_of_menu::menu_uuid;

pub struct MenuRepo {
    conn: database::PoolConnection,
}

impl MenuRepo {
    pub fn default() -> Self {
        let conn = database::pool();
        MenuRepo { conn }
    }

    pub fn new(conn: database::PoolConnection) -> Self {
        MenuRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysMenu, anyhow::Error> {
        let ret = sys_menu.filter(id.eq(i))
            .select(SysMenu::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn update(&mut self, d: SysMenu) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::update(sys_menu.filter(id.eq(d.id)))
            .set(&d)
            .execute(self.conn.deref_mut()).optional()?;
        Ok(ret)
    }

    pub fn create(&mut self, d: SysMenu) -> Result<SysMenu, anyhow::Error> {
        diesel::insert_into(sys_menu::table).values(&d).execute(self.conn.deref_mut())?;
        Ok(d)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_menu.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: i64, size: i64) -> Result<(Vec<SysMenu>, i64), anyhow::Error> {
        let offset = size * (page - 1);
        let query_result = sys_menu.select(SysMenu::as_select()).limit(size).offset(offset).load::<SysMenu>(self.conn.deref_mut())?;
        let total_count = sys_menu.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysMenu> = query_result.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }

    pub fn get_by_role_uuids(&mut self, ids: Vec<String>) -> Result<Vec<(String,SysMenu)>, anyhow::Error> {
        use crate::database::schema::sys_role_of_menu::role_uuid;
        let menus = sys_role_of_menu
            .filter(role_uuid.eq_any(ids))
            .inner_join(sys_menu.on(uuid.eq(menu_uuid)))
            .select((role_uuid, SysMenu::as_select()))//需要将role_id也带出来
            .load::<(String, SysMenu)>(self.conn.deref_mut())?;
        Ok(menus)

        // use crate::database::schema::sys_role_of_menu::role_uuid;
        // let connection = database::establish_connection();
        // let mut conn = LoggingConnection::new(connection);
        // let menus = sys_role_of_menu
        //     .filter(role_uuid.eq_any(ids))
        //     .inner_join(sys_menu.on(uuid.eq(menu_uuid)))
        //     .select((role_uuid, SysMenu::as_select()))//需要将role_id也带出来
        //     .load::<(String, SysMenu)>(&mut conn)?;
        // // Ok(menus)
        // let ret: Vec<(String,SysMenu)> = Vec::new();
        // Ok(ret)
    }
}