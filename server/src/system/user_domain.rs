use std::error::Error;
use crate::system::models::SysUser;
use crate::system::user_handler::{CreateUser, Delete, UpdateUser};
use crate::system::user_repo::UserRepo;

pub struct UserDomain {
    repo: UserRepo,
}

impl UserDomain {
    
    pub fn new(repo: UserRepo) -> Self {
        UserDomain { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysUser, Box<dyn Error>> {
        match self.repo.get_by_id(i) {
            Ok(user) => Ok(user),
            Err(e) => Err(format!("Error retrieving user: {}", e).into()),
        }
    }

    pub fn update(&mut self, u: UpdateUser) -> Result<SysUser,diesel::result::Error> {
        self.repo.update(u)
    }

    pub fn create(&mut self, u: CreateUser) -> Result<SysUser, diesel::result::Error> {
        self.repo.create(u)
    }

    pub fn delete(&mut self, d: Delete) {
        self.repo.delete_by_ids(d.ids)
    }
}