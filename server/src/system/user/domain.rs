use std::error::Error;
use std::io::ErrorKind;
use anyhow::bail;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::common::{request, response};
use crate::database::schema::sys_user::password;
use crate::system::user::model::SysUser;
use crate::system::user::repo::{UserRepo, UserRepoError};
use crate::system::user::request::{CreateUser, UpdateUser};
use crate::util;

pub struct UserDomain {
    repo: UserRepo,
}

impl UserDomain {
    
    pub fn new(repo: UserRepo) -> Self {
        UserDomain { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysUser, anyhow::Error> {
        match self.repo.get_by_id(i) {
            Ok(u) => Ok(u),
            Err(e) => bail!(e),
        }
    }
    // pub fn authorize(&mut self, username: String, pwd: String) -> Result<SysUser, AuthError> {
    //     // Check the user credentials from a database
    //     let response = self.repo.get_by_username(username.as_str());
    //     // Decrypt the password first
    //     let decoded_password = "$2b$12$n4dpJplhF9Di3n8dk7cjT.B/Uc5YGXLQUaLeYJdSEcDRX4we7XI66";
    //
    //     match response {
    //         Ok(user) => {
    //             // Check if the decrypted password matches the stored password
    //             if verify(&decoded_password, &user.password).is_ok() {
    //                 Ok(user)
    //             } else {
    //                 Err(Box::new(Error::new(<dyn std::error::Error>::description(&ErrorKind::Other), "Incorrect password.")) as Box<dyn Error>)
    //             }
    //         }
    //         Err(_) => Err(Box::new(Error::new(<dyn std::error::Error>::description(&ErrorKind::Other), "Username not found.")) as Box<dyn Error>),
    //     }
    // }



    fn decrypt_password(pwd: &str) -> Result<String, Box<dyn Error>> {
        // Decrypt the password here
        // ...
        Ok("$2b$12$n4dpJplhF9Di3n8dk7cjT.B/Uc5YGXLQUaLeYJdSEcDRX4we7XI66".parse().unwrap())
    }

    // pub fn page(&mut self, r: request::Page) -> Result<response::PageResponse<SysUser>, Box<dyn Error>> {
    //     match self.repo.page(r.page, r.size) {
    //         Ok((records, total)) => {
    //             let response = response::PageResponse::new(records, r.page, r.size, total);
    //             Ok(response)
    //         },
    //         Err(e) => Err(format!("Error retrieving user: {}", e).into()),
    //     }
    // }
    //
    // pub fn create(&mut self, u: CreateUser) -> Result<SysUser,Box<dyn Error>> {
    //     let mut user: SysUser = u.into();
    //     //密码加密
    //     // let password = user.password.unwrap_or_else(|| "123456".to_string());
    //     let hashed_password = match hash(user.password.to_string(), DEFAULT_COST) {
    //         Ok(hashed) => hashed,
    //         Err(_) => return Err(Box::try_from("Failed to hash password.".to_string()).unwrap()),
    //     };
    //     user.password = hashed_password;
    //     match self.repo.create(user) {
    //         Ok(user) => Ok(user),
    //         Err(e) => Err(format!("Error create user: {}", e).into()),
    //     }
    // }
    //
    // pub fn update(&mut self, u: UpdateUser) -> Result<(),Box<dyn Error>> {
    //     let user: SysUser = u.into();
    //     match self.repo.update(user) {
    //         Ok(Some(update)) if update > 0 => Ok(()),
    //         Ok(_) => Err(format!("No user was update").into()),
    //         Err(e) => Err(format!("Error update user: {}", e).into()),
    //     }
    // }
    //
    // pub fn delete(&mut self, d: request::Delete) -> Result<(),Box<dyn Error>> {
    //     match self.repo.delete_by_ids(d.ids) {
    //         Ok(Some(deleted)) if deleted > 0 => Ok(()),
    //         Ok(_) => Err(format!("No user was deleted").into()),
    //         Err(e) => Err(format!("Error delete user by ids: {}", e).into()),
    //     }
    // }
}


impl From<CreateUser> for SysUser {
    fn from(user: CreateUser) -> SysUser {
        SysUser {
            id: 0,
            uuid: util::uuid(),
            username: user.username,
            password: user.password,
            name: user.name,
            email: user.email,
            status: 0,
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
            uuid: "".to_string(),
            username: user.username,
            password: user.password,
            name: user.name,
            email: user.email,
            status: 0,
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