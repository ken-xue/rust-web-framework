use std::collections::HashMap;
use anyhow::bail;
use crate::common::{request, response};
use crate::{common, system};
use crate::database::schema::sys_role_of_menu::dsl::sys_role_of_menu;
use crate::system::menu;
use crate::system::menu::model::SysRoleOfMenu;
use crate::system::menu::response::MenuResponse;
use crate::system::role::response::RoleResponse;
use crate::system::role::model::SysRole;
use crate::system::role::repo::RoleRepo;
use crate::system::role::request::{AddRole,PageRole, UpdateRole };
use crate::util::uuid;

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
        Ok(self.repo.get_by_id(i)?.into())
    }

    pub fn page(&mut self, r: PageRole) -> Result<response::PageResponse<RoleResponse>, anyhow::Error> {
        self.repo.page(r.clone()).map(|(records, total)|
            response::PageResponse::<RoleResponse>::new(
                records.into_iter().map(RoleResponse::from).collect(), r.page, r.page_size, total))
    }

    pub fn list(&mut self) -> Result<Vec<RoleResponse>, anyhow::Error> {
        Ok(self.repo.list()?.into_iter().map(|d| RoleResponse::from(d)).collect())
    }

    pub fn add(&mut self, u: AddRole) -> Result<RoleResponse,anyhow::Error> {
        let u_clone = u.clone();
        let role: SysRole = u.into();
        //添加菜单关系
        let mut role_of_menus:Vec<SysRoleOfMenu> = Vec::new();
        if let Some(menus) = u_clone.menus {
            for menu in menus {
                let of = SysRoleOfMenu{
                    id: 0,
                    uuid: uuid(),
                    role_uuid: role.uuid.to_string(),
                    menu_uuid: menu.to_string(),
                    creator: Some(common::current_username()),
                    modifier: Some(common::current_username()),
                    gmt_create: Default::default(),
                    gmt_modified: Default::default(),
                    deleted: false,
                };
                role_of_menus.push(of);
            };
        }
        //添加绑定关系
        menu::repo::MenuRepo::default().add_role_of_menus(role_of_menus)?;
        //添加角色
        Ok(self.repo.add(role)?.into())
    }

    pub fn update(&mut self, u: UpdateRole) -> Result<usize,anyhow::Error> {
        //更新菜单
        // sys_role_of_menu::up
        //更新关系
        Ok(self.repo.update(u.into())?.unwrap_or(0))
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<usize,anyhow::Error> {
        Ok(self.repo.delete_by_ids(d.ids)?.unwrap_or(0))
    }

    //给角色的menus字段查询对应的值，填充到roles中
    pub fn fill_menus_for_roles(&mut self, mut roles: Vec<RoleResponse>) -> Result<Vec<RoleResponse>, anyhow::Error> {
        //是否需要查询角色对应的权限
        let role_ids: Vec<String> = roles.iter().map(|role| role.uuid.clone()).collect();
        let menus = menu::service::MenuService::default().get_by_role_uuids(role_ids)?;
        //根据menus的role_uuid赋值给每个roles的menus字段vec
        let mut menu_map: HashMap<String, Vec<MenuResponse>> = HashMap::new();
        for menu in menus {
            let menu_uuids: &mut Vec<MenuResponse> = menu_map.entry(menu.role_uuid.clone().unwrap()).or_insert(Vec::new());
            menu_uuids.push(menu);
        }
        //赋值给roles
        for mut role_response in &mut roles {
            let menus = menu_map.get(&role_response.uuid).map_or(Vec::new(), |v: &Vec<MenuResponse>| v.clone());
            role_response.menus = Option::from(menus)
        }
        Ok(roles)
    }

    pub fn get_by_user_uuid(&mut self, uid: String) -> Result<Vec<RoleResponse>, anyhow::Error> {
        //查询所属角色
        let roles = self.repo.get_by_user_uuid(uid)?;
        let mut ret: Vec<RoleResponse> = roles.into_iter().map(|d| RoleResponse::from(d)).collect();
        Ok(ret)
    }
}