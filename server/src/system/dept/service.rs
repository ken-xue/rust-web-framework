use std::collections::HashMap;
use anyhow::bail;
use crate::common::{request, response};
use crate::system::dept::response::DeptResponse;
use crate::system::dept::model::SysDept;
use crate::system::dept::repo::DeptRepo;
use crate::system::dept::request::{AddDept,PageDept, UpdateDept };

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
        Ok(self.repo.get_by_id(i)?.into())
    }

    pub fn page(&mut self, r: PageDept) -> Result<response::PageResponse<DeptResponse>, anyhow::Error> {
        self.repo.page(r.clone()).map(|(records, total)|
            response::PageResponse::<DeptResponse>::new(
                records.into_iter().map(DeptResponse::from).collect(), r.page, r.page_size, total))
    }

    pub fn list(&mut self) -> Result<Vec<DeptResponse>, anyhow::Error> {
        let list = self.repo.list()?.into_iter().map(|d| DeptResponse::from(d)).collect();
        Ok(self.tree(list)?)
    }

    pub fn add(&mut self, u: AddDept) -> Result<DeptResponse,anyhow::Error> {
        Ok(self.repo.add(u.into())?.into())
    }

    pub fn update(&mut self, u: UpdateDept) -> Result<usize,anyhow::Error> {
        Ok(self.repo.update(u.into())?.unwrap_or(0))
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<usize,anyhow::Error> {
        Ok(self.repo.delete_by_ids(d.ids)?.unwrap_or(0))
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