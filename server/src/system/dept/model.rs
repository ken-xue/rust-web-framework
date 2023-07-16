use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

// power by rwf : https://github.com/ken-xue/rust-web-framework/tree/main/server/crates/code
// template engine by handlebars : https://docs.rs/handlebars/4.3.7/handlebars/

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_dept"]
//部门表 //2023-07-16T02:24:14
pub struct SysDept {
    pub id: u64,//主键
    pub uuid: String,//uuid
    pub parent_uuid: Option<String>,//父uuid
    pub name: String,//名称
    pub order: Option<i32>,//排序
    pub remark: Option<String>,//备注
    pub status: Option<String>,//状态
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}