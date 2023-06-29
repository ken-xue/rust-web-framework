use std::error::Error;
use bcrypt::{DEFAULT_COST, hash};
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::common::{request, response};
use crate::database;
use crate::system::user::model::SysUser;
use crate::system::user::repo::UserRepo;
use crate::system::user::request::{CreateUser, UpdateUser};
use crate::util;

lazy_static! {
    // UserDomain 的全局单例
    pub static ref USER_DOMAIN: Mutex<UserDomain> = {
        let repo = UserRepo::new(database::pool());
        let domain = UserDomain::new(repo);
        Mutex::new(domain)
    };
}

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
        let mut user: SysUser = u.into();
        //密码加密
        let password = user.password.unwrap_or_else(|| "123456".to_string());
        let hashed_password = match hash(password, DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(_) => return Err(Box::try_from("Failed to hash password.".to_string()).unwrap()),
        };
        user.password = Option::from(hashed_password);
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