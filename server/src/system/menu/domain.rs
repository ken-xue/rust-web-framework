use std::error::Error;
use crate::common::{request, response};
use crate::system::menu::model::SysMenu;
use crate::system::menu::repo::MenuRepo;
use crate::system::menu::request::{CreateMenu, UpdateMenu };
use crate::util;

pub struct MenuDomain {
    repo: MenuRepo,
}

impl MenuDomain {
    
    pub fn new(repo: MenuRepo) -> Self {
        MenuDomain { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysMenu, Box<dyn Error>> {
        match self.repo.get_by_id(i) {
            Ok(d) => Ok(d),
            Err(e) => Err(format!("Error retrieving SysMenu: {}", e).into()),
        }
    }

    pub fn page(&mut self, r: request::Page) -> Result<response::PageResponse<SysMenu>, Box<dyn Error>> {
        match self.repo.page(r.page, r.size) {
            Ok((records, total)) => {
                let response = response::PageResponse::new(records, r.page, r.size, total);
                Ok(response)
            },
            Err(e) => Err(format!("Error retrieving SysMenu: {}", e).into()),
        }
    }

    pub fn create(&mut self, u: CreateMenu) -> Result<SysMenu,Box<dyn Error>> {
        let d: SysMenu = u.into();
        match self.repo.create(d) {
            Ok(d) => Ok(d),
            Err(e) => Err(format!("Error create SysMenu: {}", e).into()),
        }
    }

    pub fn update(&mut self, u: UpdateMenu) -> Result<(),Box<dyn Error>> {
        let d: SysMenu = u.into();
        match self.repo.update(d) {
            Ok(Some(update)) if update > 0 => Ok(()),
            Ok(_) => Err(format!("No SysMenu was update").into()),
            Err(e) => Err(format!("Error update SysMenu: {}", e).into()),
        }
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<(),Box<dyn Error>> {
        match self.repo.delete_by_ids(d.ids) {
            Ok(Some(deleted)) if deleted > 0 => Ok(()),
            Ok(_) => Err(format!("No SysMenu was deleted").into()),
            Err(e) => Err(format!("Error delete SysMenu by ids: {}", e).into()),
        }
    }
}


impl From<CreateMenu> for SysMenu {
    fn from(req: CreateMenu) -> SysMenu {
        SysMenu {
            id: req.id,//主键
            uuid: req.uuid,//uuid
            menu_parent_uuid: req.menu_parent_uuid,//父菜单uuid
            menu_name: req.menu_name,//菜单名
            menu_url: req.menu_url,//菜单url
            menu_perms: req.menu_perms,//授权标识
            menu_type: req.menu_type,//0:目录 1:菜单 2:按钮
            menu_icon: req.menu_icon,//图标
            menu_order: req.menu_order,//排序
            menu_remark: req.menu_remark,//备注
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
        }
    }
}

impl From<UpdateMenu> for SysMenu {
    fn from(req: UpdateMenu) -> SysMenu {
        SysMenu {
            id: req.id,//主键
            uuid: req.uuid,//uuid
            menu_parent_uuid: req.menu_parent_uuid,//父菜单uuid
            menu_name: req.menu_name,//菜单名
            menu_url: req.menu_url,//菜单url
            menu_perms: req.menu_perms,//授权标识
            menu_type: req.menu_type,//0:目录 1:菜单 2:按钮
            menu_icon: req.menu_icon,//图标
            menu_order: req.menu_order,//排序
            menu_remark: req.menu_remark,//备注
            creator: req.creator,//创建人
            modifier: req.modifier,//修改人
            gmt_create: req.gmt_create,//创建时间
            gmt_modified: req.gmt_modified,//修改时间
            deleted: req.deleted,//逻辑删除
        }
    }
}