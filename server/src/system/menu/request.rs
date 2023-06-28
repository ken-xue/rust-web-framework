use serde::{Deserialize};
use validator::{Validate};

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateMenu {
    pub id: u64,//主键
    pub uuid: Option<String>,//uuid
    pub menu_parent_uuid: Option<String>,//父菜单uuid
    pub menu_name: String,//菜单名
    pub menu_url: Option<String>,//菜单url
    pub menu_perms: Option<String>,//授权标识
    pub menu_type: Option<String>,//0:目录 1:菜单 2:按钮
    pub menu_icon: Option<String>,//图标
    pub menu_order: Option<i32>,//排序
    pub menu_remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateMenu {
    //#[validate(length(min = 1, message = "Can not be empty"))]
    pub id: u64,//主键
    pub uuid: Option<String>,//uuid
    pub menu_parent_uuid: Option<String>,//父菜单uuid
    pub menu_name: String,//菜单名
    pub menu_url: Option<String>,//菜单url
    pub menu_perms: Option<String>,//授权标识
    pub menu_type: Option<String>,//0:目录 1:菜单 2:按钮
    pub menu_icon: Option<String>,//图标
    pub menu_order: Option<i32>,//排序
    pub menu_remark: Option<String>,//备注
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
}