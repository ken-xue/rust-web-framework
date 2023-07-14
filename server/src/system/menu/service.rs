use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::process::id;
use std::sync::Arc;
use anyhow::bail;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
use crate::common::{request, response};
use crate::common::request::Page;
use crate::database::schema::sys_menu::dsl::sys_menu;
use crate::database::schema::sys_menu::uuid;
use crate::database::schema::sys_role_of_menu::dsl::sys_role_of_menu;
use crate::system::{auth, role, user};
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

    pub fn list(&mut self) -> Result<Vec<MenuResponse>, anyhow::Error> {
        let username = auth::CURRENT_USER.with(|cell| {
            cell.borrow().clone()
        });
        //查询角色和菜单
        let user = user::service::UserService::default().get_by_username(username.unwrap())?;
        let mut menu_set: HashSet<MenuResponse> = HashSet::new();
        if let Some(roles) = &user.roles {
            for role in roles.iter() {
                if let Some(menus) = &role.menus {
                    menu_set.extend(menus.iter().cloned());
                }
            }
        }
        let menus: Vec<MenuResponse> = menu_set.into_iter().collect();
        //构建成菜单树
        let tree_menus = self.tree(menus)?;
        //
        Ok(tree_menus)
    }

    // 构建菜单树
    pub fn tree(&mut self, mut menus: Vec<MenuResponse>) -> Result<Vec<MenuResponse>, anyhow::Error> {
        let mut map: HashMap<String, MenuResponse> = HashMap::new();
        for menu in menus.iter() {
            map.insert(menu.uuid.clone(), menu.clone());
        }
        let mut list: Vec<&MenuResponse> = Vec::new();
        for menu in menus.iter_mut() {
            if menu.parent_uuid.is_none() || menu.parent_uuid.clone().unwrap().eq("0"){
                list.push(menu);
            } else {
                let parent_uuid = menu.parent_uuid.as_ref().unwrap();
                if let Some(parent) = map.get_mut(parent_uuid) {
                    if parent.children.is_none() {
                        parent.children = Some(Vec::new());
                    }
                    parent.children.as_mut().unwrap().push(menu.clone());
                }
            }
        }
        let mut result: Vec<MenuResponse> = Vec::new();
        for menu in map.values() {
            if menu.parent_uuid.is_none() || menu.parent_uuid.as_ref().unwrap() == "0" {
                result.push(menu.clone());
            }
        }
        result.sort_by(|a, b| a.order.unwrap_or(1000).cmp(&b.order.unwrap_or(1000)));
        Ok(result)
    }
}