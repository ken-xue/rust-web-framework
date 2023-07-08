use std::process::id;
use anyhow::bail;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
use crate::common::{request, response};
use crate::database::schema::sys_menu::dsl::sys_menu;
use crate::database::schema::sys_menu::uuid;
use crate::database::schema::sys_role_of_menu::dsl::sys_role_of_menu;
use crate::system::menu::response::MenuResponse;
use crate::system::menu::model::SysMenu;
use crate::system::menu::repo::MenuRepo;
use crate::system::menu::request::{CreateMenu, UpdateMenu };

pub struct MenuService {
    repo: MenuRepo,
}

impl MenuService {

    pub fn default() -> Self {
        let repo = MenuRepo::default();
        MenuService { repo }
    }

    pub fn new(repo: MenuRepo) -> Self {
        MenuService { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<MenuResponse, anyhow::Error> {
        let resp = self.repo.get_by_id(i)?;
        Ok(resp.into())
    }

    pub fn page(&mut self, r: request::Page) -> Result<response::PageResponse<MenuResponse>, anyhow::Error> {
        match self.repo.page(r.page, r.size) {
            Ok((records, total)) => {
                let list = records.into_iter().map(|d| MenuResponse::from(d)).collect();
                let response = response::PageResponse::new(list, r.page, r.size, total);
                Ok(response)
            }
            Err(e) => bail!(e),
        }
    }

    pub fn create(&mut self, u: CreateMenu) -> Result<MenuResponse,anyhow::Error> {
        let d: SysMenu = u.into();
        match self.repo.create(d) {
            Ok(d) => Ok(d.into()),
            Err(e) => bail!("Error create SysMenu: {}", e),
        }
    }

    pub fn update(&mut self, u: UpdateMenu) -> Result<(),anyhow::Error> {
        let d: SysMenu = u.into();
        match self.repo.update(d) {
            Ok(Some(update)) if update > 0 => Ok(()),
            Ok(_) => bail!("No SysMenu was update"),
            Err(e) => bail!("Error update SysMenu: {}", e),
        }
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<(),anyhow::Error> {
        match self.repo.delete_by_ids(d.ids) {
            Ok(Some(deleted)) if deleted > 0 => Ok(()),
            Ok(_) => bail!("No SysMenu was deleted"),
            Err(e) => bail!("Error delete SysMenu by ids: {}", e),
        }
    }

    pub fn get_by_role_uuids(&mut self, ids: Vec<String>) -> Result<Vec<MenuResponse>,anyhow::Error> {
        let menus = self.repo.get_by_role_uuids(ids)?;
        let ret: Vec<MenuResponse> = menus.into_iter().map(|(k,v)| <SysMenu as Into<MenuResponse>>::into(v).set_menu_uuid(k)).collect();
        Ok(ret)
    }
}