use crate::database::schema::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize};

// power by rwf : https://github.com/ken-xue/rust-web-framework/tree/main/server/crates/code
// template engine by handlebars : https://docs.rs/handlebars/4.3.7/handlebars/

#[derive(Debug,Serialize,Queryable,Identifiable,Selectable,Insertable,AsChangeset)]
#[table_name = "sys_config"]
//系统配置表 //2023-07-18T06:37:33
pub struct SysConfig {
    pub id: u64,//主键
    pub uuid: String,//唯一标示做关联
    pub name: Option<String>,//名字
    pub config: Option<String>,//配置信息
    pub remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
    pub deletable: String,//是否可删除
    pub editable: Option<String>,//是否可编辑
}