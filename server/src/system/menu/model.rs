use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

// power by rwf : https://github.com/ken-xue/rust-web-framework/tree/main/server/crates/code
// template engine by handlebars : https://docs.rs/handlebars/4.3.7/handlebars/

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_menu"]
//菜单表 //2023-07-01T02:59:49
pub struct SysMenu {
    pub id: u64,//主键
    pub uuid: String,//uuid
    pub status: String,//status
    pub parent_uuid: Option<String>,//父菜单uuid
    pub name: String,//菜单名
    pub path: Option<String>,//菜单url
    pub permission: Option<String>,//授权标识
    pub menu_type: Option<String>,//0:目录 1:菜单 2:按钮
    pub icon: Option<String>,//图标
    pub order: Option<i32>,//排序
    pub remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub component: Option<String>,//修改人
    pub redirect: Option<String>,//修改人
    pub title: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_role_of_menu"]
//角色表 //2023-07-01T02:59:49
pub struct SysRoleOfMenu {
    pub id: u64,//主键
    pub uuid: String,//uuid
    pub role_uuid: String,//uuid
    pub menu_uuid: String,//uuid
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}