use serde::{Serialize};
use crate::system::config::model::SysConfig;

#[derive(Debug,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
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

impl From<SysConfig> for ConfigResponse {
    fn from(req: SysConfig) -> ConfigResponse {
        ConfigResponse {
            id: req.id,//主键
            uuid: req.uuid,//唯一标示做关联
            name: req.name,//名字
            config: req.config,//配置信息
            remark: req.remark,//备注
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
            deletable: req.deletable,//是否可删除
            editable: req.editable,//是否可编辑
        }
    }
}