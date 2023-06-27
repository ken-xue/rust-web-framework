use std::error::Error;
use crate::common::{request, response};
use crate::system::user::model::SysUser;
use crate::system::user::repo::UserRepo;
use crate::system::user::request::{CreateUser, UpdateUser};
use crate::util;

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

    pub fn page(&mut self, r: request::Page) -> Result<response::PageResponse<SysUser>, Box<dyn Error>> {
        match self.repo.page(r.page, r.size) {
            Ok((records, total)) => {
                let response = response::PageResponse::new(records, r.page, r.size, total);
                Ok(response)
            },
            Err(e) => Err(format!("Error retrieving user: {}", e).into()),
        }
    }

    pub fn create(&mut self, u: CreateUser) -> Result<SysUser,Box<dyn Error>> {
        let user: SysUser = u.into();
        match self.repo.create(user) {
            Ok(user) => Ok(user),
            Err(e) => Err(format!("Error create user: {}", e).into()),
        }
    }

    pub fn update(&mut self, u: UpdateUser) -> Result<(),Box<dyn Error>> {
        let user: SysUser = u.into();
        match self.repo.update(user) {
            Ok(Some(update)) if update > 0 => Ok(()),
            Ok(_) => Err(format!("No user was update").into()),
            Err(e) => Err(format!("Error update user: {}", e).into()),
        }
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<(),Box<dyn Error>> {
        match self.repo.delete_by_ids(d.ids) {
            Ok(Some(deleted)) if deleted > 0 => Ok(()),
            Ok(_) => Err(format!("No user was deleted").into()),
            Err(e) => Err(format!("Error delete user by ids: {}", e).into()),
        }
    }
}


impl From<CreateUser> for SysUser {
    fn from(user: CreateUser) -> SysUser {
        SysUser {
            id: 0,
            uuid: Some(util::uuid()),
            account: Option::from(user.account),
            password: Option::from(user.password),
            name: Some(user.name),
            email: Option::from(user.email),
            status: None,
            creator: None,
            modifier: None,
            gmt_create: Default::default(),
            gmt_modified: Default::default(),
            avatar: None,
            deleted: false,
        }
    }
}

impl From<UpdateUser> for SysUser {
    fn from(user: UpdateUser) -> SysUser {
        SysUser {
            id: user.id,
            uuid: None,
            account: Option::from(user.account),
            password: Option::from(user.password),
            name: Some(user.name),
            email: Option::from(user.email),
            status: None,
            creator: None,
            modifier: None,
            gmt_create: Default::default(),
            gmt_modified: Default::default(),
            avatar: None,
            deleted: false,
        }
    }
}


#[derive(Debug)]
enum UserStatus {
    #[allow(dead_code)]
    Normal,
    #[allow(dead_code)]
    Blocked,
}