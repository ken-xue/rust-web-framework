use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_menu"]
pub struct SysMenu {
    pub id: u64,
    pub uuid: Option<String>,
    pub menu_parent_uuid: Option<String>,
    pub menu_name: Option<String>,
    pub menu_url: Option<String>,
    pub menu_perms: Option<String>,
    pub menu_type: Option<String>,
    pub menu_icon: Option<String>,
    pub menu_order: Option<i32>,
    pub menu_remark: Option<String>,
    pub creator: Option<String>,
    pub modifier: Option<String>,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_role_of_menu"]
pub struct SysRoleOfMenu {
    pub id: u64,
    pub uuid: Option<String>,
    pub role_uuid: Option<String>,
    pub menu_uuid: Option<String>,
    pub creator: Option<String>,
    pub modifier: Option<String>,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub deleted: bool,
}