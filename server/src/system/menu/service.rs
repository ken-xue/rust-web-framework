use std::collections::{HashMap};
use anyhow::{anyhow, bail};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, SelectableHelper};
use crate::common::{request, response};
use crate::common::request::Page;
use crate::system::menu::response::MenuResponse;
use crate::system::menu::model::SysMenu;
use crate::system::menu::repo::MenuRepo;
use crate::system::menu::request::{AddMenu, PageMenu, UpdateMenu};

pub enum MenuType {
    DIR,
    MENU,
    BUTTON
}

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
        Ok(self.repo.get_by_id(i)?.into())
    }

    pub fn page(&mut self, r: PageMenu) -> Result<response::PageResponse<MenuResponse>, anyhow::Error> {
        self.repo.page(r.clone()).map(|(records, total)|
            response::PageResponse::<MenuResponse>::new(
                records.into_iter().map(MenuResponse::from).collect(), r.page, r.page_size, total))
    }

    pub fn add(&mut self, u: AddMenu) -> Result<MenuResponse,anyhow::Error> {
        Ok(self.repo.add(u.into())?.into())
    }

    pub fn update(&mut self, u: UpdateMenu) -> Result<usize,anyhow::Error> {
        Ok(self.repo.update(u.into())?.unwrap_or(0))
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<usize,anyhow::Error> {
        Ok(self.repo.delete_by_ids(d.ids)?.unwrap_or(0))
    }

    pub fn get_by_role_uuids(&mut self, ids: Vec<String>) -> Result<Vec<MenuResponse>,anyhow::Error> {
        let menus = self.repo.get_by_role_uuids(ids)?;
        let ret: Vec<MenuResponse> = menus.into_iter().map(|(k,v)| <SysMenu as Into<MenuResponse>>::into(v).set_menu_uuid(k)).collect();
        Ok(ret)
    }

    pub fn list(&mut self) -> Result<Vec<MenuResponse>, anyhow::Error> {
        let menus = self.repo.list()?.into_iter().map(|d| MenuResponse::from(d)).collect();
        let ret = self.tree(menus)?;
        Ok(ret)
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