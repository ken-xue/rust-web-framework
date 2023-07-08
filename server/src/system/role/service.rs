use std::collections::HashMap;
use anyhow::bail;
use crate::common::{request, response};
use crate::system;
use crate::system::menu;
use crate::system::menu::response::MenuResponse;
use crate::system::role::response::RoleResponse;
use crate::system::role::model::SysRole;
use crate::system::role::repo::RoleRepo;
use crate::system::role::request::{CreateRole, UpdateRole };

pub struct RoleService {
    repo: RoleRepo,
}

impl RoleService {

    pub fn default() -> Self {
        let repo = RoleRepo::default();
        RoleService { repo }
    }

    pub fn new(repo: RoleRepo) -> Self {
        RoleService { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<RoleResponse, anyhow::Error> {
        let resp = self.repo.get_by_id(i)?;
        Ok(resp.into())
    }

    pub fn page(&mut self, r: request::Page) -> Result<response::PageResponse<RoleResponse>, anyhow::Error> {
        match self.repo.page(r.page, r.size) {
            Ok((records, total)) => {
                let list = records.into_iter().map(|d| RoleResponse::from(d)).collect();
                let response = response::PageResponse::new(list, r.page, r.size, total);
                Ok(response)
            }
            Err(e) => bail!(e),
        }
    }

    pub fn create(&mut self, u: CreateRole) -> Result<RoleResponse,anyhow::Error> {
        let d: SysRole = u.into();
        match self.repo.create(d) {
            Ok(d) => Ok(d.into()),
            Err(e) => bail!("Error create SysRole: {}", e),
        }
    }

    pub fn update(&mut self, u: UpdateRole) -> Result<(),anyhow::Error> {
        let d: SysRole = u.into();
        match self.repo.update(d) {
            Ok(Some(update)) if update > 0 => Ok(()),
            Ok(_) => bail!("No SysRole was update"),
            Err(e) => bail!("Error update SysRole: {}", e),
        }
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<(),anyhow::Error> {
        match self.repo.delete_by_ids(d.ids) {
            Ok(Some(deleted)) if deleted > 0 => Ok(()),
            Ok(_) => bail!("No SysRole was deleted"),
            Err(e) => bail!("Error delete SysRole by ids: {}", e),
        }
    }

    pub fn get_by_user_uuid(&mut self, uid: String) -> Result<Vec<RoleResponse>, anyhow::Error> {
        //查询所属角色
        let roles = self.repo.get_by_user_uuid(uid)?;
        let role_ids: Vec<String> = roles.iter().map(|role| role.uuid.clone()).collect();
        let ret: Vec<RoleResponse> = roles.into_iter().map(|d| RoleResponse::from(d)).collect();
        //查询权限
        let menus = menu::service::MenuService::default().get_by_role_uuids(role_ids)?;
        //根据menus的role_uuid赋值给每个roles的menus字段vec
        let mut menu_map: HashMap<String, Vec<MenuResponse>> = HashMap::new();
        for menu in menus {
            let menu_uuids: &mut Vec<MenuResponse> = menu_map.entry(menu.role_uuid.unwrap()).or_insert(Vec::new());
            menu_uuids.push(menu);
        }
        //赋值给roles
        for mut role_response in ret.into_iter() {
            let menus = menu_map.get(&role_response.uuid).map_or(Vec::new(), |v:&Vec<MenuResponse>| v.clone());
            role_response.menus = Option::from(menus)
        }
        Ok(ret)
    }
}