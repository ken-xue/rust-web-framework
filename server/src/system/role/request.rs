use serde::{Deserialize};
use validator::{Validate};
use crate::system::role::model::SysRole;

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateRole {
    pub id: u64,//主键
    pub uuid: Option<String>,//uuid
    pub name: Option<String>,//角色名
    pub remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateRole {
    //#[validate(length(min = 1, message = "Can not be empty"))]
    pub id: u64,//主键
    pub uuid: Option<String>,//uuid
    pub name: Option<String>,//角色名
    pub remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}

impl From<CreateRole> for SysRole {
    fn from(req: CreateRole) -> SysRole {
        SysRole {
            id: req.id,//主键
            uuid: req.uuid,//uuid
            name: req.name,//角色名
            remark: req.remark,//备注
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
        }
    }
}

impl From<UpdateRole> for SysRole {
    fn from(req: UpdateRole) -> SysRole {
        SysRole {
            id: req.id,//主键
            uuid: req.uuid,//uuid
            name: req.name,//角色名
            remark: req.remark,//备注
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
        }
    }
}