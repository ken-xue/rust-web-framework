use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

// power by rwf : https://github.com/ken-xue/rust-web-framework/tree/main/server/crates/code

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_user"]
//用户表 //2023-06-30T08:44:10
pub struct SysUser {
    pub id: u64,//主键
    pub uuid: String,//唯一标示做关联
    pub username: String,//账号
    pub password: String,//密码
    pub name: String,//名字
    pub email: String,//邮箱
    pub status: i32,//状态
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
    pub avatar: Option<String>,//头像
}