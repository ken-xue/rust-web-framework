use serde::{Serialize};
use crate::system::menu::model::SysMenu;

#[derive(Debug,Serialize,Clone,Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MenuResponse {
    pub id: u64,//主键
    pub uuid: String,//uuid
    pub role_uuid: Option<String>,//角色uuid
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
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub deleted: bool,//逻辑删除
    pub children: Option<Vec<MenuResponse>>,//逻辑删除
    pub component: Option<String>,//修改人
    pub redirect: Option<String>,//修改人
    pub title: Option<String>,//修改人
    pub status: String,//修改人
}

impl MenuResponse {
    pub(crate) fn set_menu_uuid(mut self, role_uuid: String) -> Self {
        self.role_uuid = Some(role_uuid);
        self
    }
}

impl From<SysMenu> for MenuResponse {
    fn from(req: SysMenu) -> MenuResponse {
        MenuResponse {
            id: req.id,//主键
            uuid: req.uuid,//uuid
            role_uuid: None,//角色uuid
            parent_uuid: req.parent_uuid,//父菜单uuid
            name: req.name,//菜单名
            path: req.path,//菜单url
            permission: req.permission,//授权标识
            menu_type: req.menu_type,//0:目录 1:菜单 2:按钮
            icon: req.icon,//图标
            order: req.order,//排序
            remark: req.remark,//备注
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
            children: None,
            component: req.component,//修改人
            redirect: req.redirect,//修改人
            title: req.title,//修改人
            status: req.status,//修改人
        }
    }
}
