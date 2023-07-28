use serde::{Deserialize};
use validator::{Validate};
use crate::common;
use crate::system::dept::model::SysDept;
use crate::util::uuid;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDept {
    pub parent_uuid: Option<String>,//父uuid
    pub name: String,//名称
    pub order: Option<i32>,//排序
    pub remark: Option<String>,//备注
    pub status: Option<String>,//状态
    pub deleted: Option<bool>,//逻辑删除
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDept {
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

#[derive(Debug, Validate, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageDept {
    pub page: i64,
    pub page_size: i64,
    pub parent_uuid: Option<String>,//父uuid
    pub name: String,//名称
    pub order: Option<i32>,//排序
    pub remark: Option<String>,//备注
    pub status: Option<String>,//状态
}

#[derive(Debug, Validate, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListDept {
    pub name: Option<String>,//名称
    pub order: Option<i32>,//排序
    pub remark: Option<String>,//备注
    pub status: Option<String>,//状态
}

impl From<AddDept> for SysDept {
    fn from(req: AddDept) -> SysDept {
        SysDept {
            id: Default::default(),//主键
            uuid: uuid(),//uuid
            parent_uuid: req.parent_uuid,//父uuid
            name: req.name,//名称
            order: req.order,//排序
            remark: req.remark,//备注
            status: req.status,//状态
            creator: Some(common::current_username()),//创建人
            modifier: Some(common::current_username()),//修改人
            gmt_create: Default::default(),//创建时间
            gmt_modified: Default::default(),//修改时间
            deleted: false,//逻辑删除
        }
    }
}

impl From<UpdateDept> for SysDept {
    fn from(req: UpdateDept) -> SysDept {
        SysDept {
            id: req.id,//主键
            uuid: req.uuid,//uuid
            parent_uuid: req.parent_uuid,//父uuid
            name: req.name,//名称
            order: req.order,//排序
            remark: req.remark,//备注
            status: req.status,//状态
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
        }
    }
}