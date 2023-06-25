use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_role"]
pub struct SysRole {
    pub id: u64,
    pub uuid: Option<String>,
    pub role_name: Option<String>,
    pub role_remark: Option<String>,
    pub creator: Option<String>,
    pub modifier: Option<String>,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub deleted: bool,
}

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_user_of_role"]
pub struct SysUserOfRole {
    pub id: u64,
    pub uuid: Option<String>,
    pub user_uuid: Option<String>,
    pub role_uuid: Option<String>,
    pub creator: Option<String>,
    pub modifier: Option<String>,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub deleted: bool,
}