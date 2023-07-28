use serde::{Deserialize};
use validator::{Validate};
use crate::system::role::model::SysRole;
use crate::{common, util};

#[derive(Debug, Validate, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddRole {
    pub name: Option<String>,//角色名
    pub remark: Option<String>,//备注
    pub menus: Option<Vec<String>>,//创建人
}

#[derive(Debug, Validate, Deserialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRole {
    pub id: u64,//主键
    pub name: Option<String>,//角色名
    pub remark: Option<String>,//备注
    pub menus: Option<Vec<String>>,//创建人
    pub deleted: Option<bool>,//创建人
}

#[derive(Debug, Validate, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageRole {
    pub page: i64,
    pub page_size: i64,
    pub name: Option<String>,//角色名
    pub remark: Option<String>,//备注
}

impl From<AddRole> for SysRole {
    fn from(req: AddRole) -> SysRole {
        SysRole {
            id: Default::default(),//主键
            uuid: util::uuid(),
            name: req.name,//角色名
            remark: req.remark,//备注
            creator: Some(common::current_username()),//创建人
            modifier: Some(common::current_username()),//修改人
            gmt_create: Default::default(),//创建时间
            gmt_modified: Default::default(),//修改时间
            deleted: false,//逻辑删除
        }
    }
}

impl From<UpdateRole> for SysRole {
    fn from(req: UpdateRole) -> SysRole {
        SysRole {
            id: req.id,//主键
            uuid: Default::default(),//uuid
            name: req.name,//角色名
            remark: req.remark,//备注
            creator: Default::default(),//创建人
            modifier: Some(common::current_username()),//修改人
            gmt_create: Default::default(),//创建时间
            gmt_modified: Default::default(),//修改时间
            deleted: false,//逻辑删除
        }
    }
}