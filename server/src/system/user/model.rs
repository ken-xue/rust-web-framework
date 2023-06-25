use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_captcha"]
pub struct SysCaptcha {
    pub id: i64,
    pub uuid: String,
    pub code: String,
    pub expire_time: Option<NaiveDateTime>,
}


#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_dictionary"]
pub struct SysDictionary {
    pub id: i64,
    pub key: Option<String>,
    pub value: Option<String>,
    pub fixed: Option<bool>,
}


#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
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

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_user"]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
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
    pub deleted: bool,
    pub avatar: Option<String>,
}