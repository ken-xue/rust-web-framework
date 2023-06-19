use std::fmt::{Error};

use crate::system::sys_model::SysUser;
use crate::system::user_handler::CreateUser;
use crate::system::user_repo::UserRepo;

pub struct UserDomain {
    repo: UserRepo,
}

impl UserDomain {
    
    pub fn new(repo: UserRepo) -> Self {
        UserDomain { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysUser, Error> {
        self.repo.get_user_by_id(i)
    }

    pub fn update(&mut self) {
        self.repo.update_user()
    }

    pub fn create(&mut self, u: CreateUser) -> Result<SysUser, Error> {
        self.repo.create_user(u)
    }
}