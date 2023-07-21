use anyhow::bail;
use crate::common::{request, response};
use crate::system::config::response::ConfigResponse;
use crate::system::config::model::SysConfig;
use crate::system::config::repo::ConfigRepo;
use crate::system::config::request::{AddConfig,PageConfig, UpdateConfig };

pub struct ConfigService {
    repo: ConfigRepo,
}

impl ConfigService {

    pub fn default() -> Self {
        let repo = ConfigRepo::default();
        ConfigService { repo }
    }

    pub fn new(repo: ConfigRepo) -> Self {
        ConfigService { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<ConfigResponse, anyhow::Error> {
        Ok(self.repo.get_by_id(i)?.into())
    }

    pub fn page(&mut self, r: PageConfig) -> Result<response::PageResponse<ConfigResponse>, anyhow::Error> {
        self.repo.page(r.clone()).map(|(records, total)|
            response::PageResponse::<ConfigResponse>::new(
                records.into_iter().map(ConfigResponse::from).collect(), r.page, r.page_size, total))
    }

    pub fn list(&mut self) -> Result<Vec<ConfigResponse>, anyhow::Error> {
        Ok(self.repo.list()?.into_iter().map(|d| ConfigResponse::from(d)).collect())
    }

    pub fn add(&mut self, u: AddConfig) -> Result<ConfigResponse,anyhow::Error> {
        Ok(self.repo.add(u.into())?.into())
    }

    pub fn update(&mut self, u: UpdateConfig) -> Result<usize,anyhow::Error> {
        Ok(self.repo.update(u.into())?.unwrap_or(0))
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<usize,anyhow::Error> {
        Ok(self.repo.delete_by_ids(d.ids)?.unwrap_or(0))
    }
}