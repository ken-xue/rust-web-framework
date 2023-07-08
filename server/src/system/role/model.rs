use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

// power by rwf : https://github.com/ken-xue/rust-web-framework/tree/main/server/crates/code
// template engine by handlebars : https://docs.rs/handlebars/4.3.7/handlebars/

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_role"]
//角色表 //2023-07-01T02:59:49
pub struct SysRole {
    pub id: u64,//主键
    pub uuid: String,//uuid
    pub name: Option<String>,//角色名
    pub remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_user_of_role"]
//角色表 //2023-07-01T02:59:49
pub struct SysUserOfRole {
    pub id: u64,//主键
    pub uuid: String,//uuid
    pub role_uuid: String,//uuid
    pub user_uuid: String,//uuid
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}