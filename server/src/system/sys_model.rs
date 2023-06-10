use crate::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "sys_captcha"]
pub struct SysCaptcha {
    pub id: i64,
    pub uuid: String,
    pub code: String,
    pub expire_time: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "sys_dictionary"]
pub struct SysDictionary {
    pub id: i64,
    pub key: Option<String>,
    pub value: Option<String>,
    pub fixed: Option<bool>,
}

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "sys_log"]
pub struct SysLog {
    pub id: i64,
    pub username: Option<String>,
    pub operation: Option<String>,
    pub method: Option<String>,
    pub params: Option<String>,
    pub execute_time: i64,
    pub ip: Option<String>,
    pub occur_time: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Identifiable)]
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
    pub deleted: Option<String>,
}

#[derive(Debug, Queryable, Identifiable)]
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
    pub deleted: Option<String>,
}

#[derive(Debug, Queryable, Identifiable)]
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
    pub deleted: Option<String>,
}

#[derive(Debug,Queryable, Identifiable,Selectable)]
#[table_name = "sys_user"]
pub struct SysUser {
    pub id: u64,
    pub uuid: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub status: Option<i32>,
    pub creator: Option<String>,
    pub modifier: Option<String>,
    pub gmt_create: NaiveDateTime,
    pub gmt_modified: NaiveDateTime,
    pub deleted: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Queryable, Identifiable)]
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
    pub deleted: Option<String>,
}