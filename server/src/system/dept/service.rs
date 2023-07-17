use std::collections::HashMap;
use anyhow::bail;
use crate::common::{request, response};
use crate::system::dept::response::DeptResponse;
use crate::system::dept::model::SysDept;
use crate::system::dept::repo::DeptRepo;
use crate::system::dept::request::{CreateDept, UpdateDept };

pub struct DeptService {
    repo: DeptRepo,
}

impl DeptService {

    pub fn default() -> Self {
        let repo = DeptRepo::default();
        DeptService { repo }
    }

    pub fn new(repo: DeptRepo) -> Self {
        DeptService { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<DeptResponse, anyhow::Error> {
        let resp = self.repo.get_by_id(i)?;
        Ok(resp.into())
    }

    pub fn page(&mut self, r: request::Page) -> Result<response::PageResponse<DeptResponse>, anyhow::Error> {
        match self.repo.page(r.page, r.page_size) {
            Ok((records, total)) => {
                let list = records.into_iter().map(|d| DeptResponse::from(d)).collect();
                let response = response::PageResponse::new(list, r.page, r.page_size, total);
                Ok(response)
            }
            Err(e) => bail!(e),
        }
    }

    pub fn list(&mut self) -> Result<Vec<DeptResponse>, anyhow::Error> {
        let list = self.repo.list()?;
        let ret = list.into_iter().map(|d| DeptResponse::from(d)).collect();
        let ret = self.tree(ret)?;
        Ok(ret)
    }

    pub fn add(&mut self, u: CreateDept) -> Result<DeptResponse,anyhow::Error> {
        let d: SysDept = u.into();
        match self.repo.add(d) {
            Ok(d) => Ok(d.into()),
            Err(e) => bail!("Error add SysDept: {}", e),
        }
    }

    pub fn update(&mut self, u: UpdateDept) -> Result<(),anyhow::Error> {
        let d: SysDept = u.into();
        match self.repo.update(d) {
            Ok(Some(update)) if update > 0 => Ok(()),
            Ok(_) => bail!("No SysDept was update"),
            Err(e) => bail!("Error update SysDept: {}", e),
        }
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<(),anyhow::Error> {
        match self.repo.delete_by_ids(d.ids) {
            Ok(Some(deleted)) if deleted > 0 => Ok(()),
            Ok(_) => bail!("No SysDept was deleted"),
            Err(e) => bail!("Error delete SysDept by ids: {}", e),
        }
    }

    // 构建菜单树
    pub fn tree(&mut self, mut menus: Vec<DeptResponse>) -> Result<Vec<DeptResponse>, anyhow::Error> {
        let mut map: HashMap<String, DeptResponse> = HashMap::new();
        for menu in menus.iter() {
            map.insert(menu.uuid.clone(), menu.clone());
        }
        let mut list: Vec<&DeptResponse> = Vec::new();
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
        let mut result: Vec<DeptResponse> = Vec::new();
        for menu in map.values() {
            if menu.parent_uuid.is_none() || menu.parent_uuid.as_ref().unwrap() == "0" {
                result.push(menu.clone());
            }
        }
        result.sort_by(|a, b| a.order.unwrap_or(1000).cmp(&b.order.unwrap_or(1000)));
        Ok(result)
    }
}