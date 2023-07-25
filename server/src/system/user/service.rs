use std::error::Error;
use anyhow::bail;
use base64::decode;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::common::{request, response};
use crate::system::{auth, role};
use crate::system::user::model::SysUser;
use crate::system::user::repo::{UserRepo};
use crate::system::user::request::{AddUser, PageUser, UpdatePassword, UpdateUser};
use crate::system::user::response::UserResponse;
use crate::util;

pub struct UserService {
    repo: UserRepo,
}

impl UserService {
    pub fn default() -> Self {
        let repo = UserRepo::default();
        UserService { repo }
    }

    pub fn new(repo: UserRepo) -> Self {
        UserService { repo }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<UserResponse, anyhow::Error> {
        Ok(self.repo.get_by_id(i)?.into())
    }

    pub fn get_by_username(&mut self, username: String) -> Result<UserResponse, anyhow::Error> {
        let user = self.repo.get_by_username(&username)?;
        let user_uuid = user.uuid.clone();
        let mut ret = UserResponse::from(user);
        // 查询角色和菜单
        let mut srv = role::service::RoleService::default();
        let roles = srv.get_by_user_uuid(user_uuid)?;
        // 填充角色包含的菜单
        let roles = srv.fill_menus_for_roles(roles)?;
        ret.roles = Some(roles);
        // 响应
        Ok(ret)
    }

    pub fn authorize(&mut self, username: String, password: String) -> Result<UserResponse, anyhow::Error> {
        // Check the user credentials from a database
        let user = self.repo.get_by_username(username.as_str())?;
        // Decrypt the password first
        let decode_base64_password = decode(password)?;
        // 解密密码
        let decoded_password = util::encrypt::default_decrypt(&decode_base64_password)?;
        // 验证密码
        if verify(&decoded_password, &user.password)? {
            let user_uuid = user.uuid.clone();
            let mut ret = UserResponse::from(user);
            // 查询角色和菜单
            let mut srv = role::service::RoleService::default();
            let roles = srv.get_by_user_uuid(user_uuid)?;
            // 填充角色包含的菜单
            let roles = srv.fill_menus_for_roles(roles)?;
            ret.roles = Some(roles);
            return Ok(ret);
        }
        bail!("Incorrect password.")
    }

    pub fn page(&mut self, r: PageUser) -> Result<response::PageResponse<UserResponse>, anyhow::Error> {
        self.repo.page(r.clone()).map(|(records, total)|
            response::PageResponse::<UserResponse>::new(
                records.into_iter().map(UserResponse::from).collect(), r.page, r.page_size, total))
    }

    pub fn add(&mut self, u: AddUser) -> Result<UserResponse, anyhow::Error> {
        let mut user: SysUser = u.into();
        //密码加密
        let hashed_password = hash(user.password.to_string(), DEFAULT_COST)?;
        user.password = hashed_password;
        Ok(self.repo.add(user)?.into())
    }

    pub fn update(&mut self, u: UpdateUser) -> Result<usize, anyhow::Error> {
        Ok(self.repo.update(u.into())?.unwrap_or(0))
    }

    pub fn delete(&mut self, d: request::Delete) -> Result<usize, anyhow::Error> {
        Ok(self.repo.delete_by_ids(d.ids)?.unwrap_or(0))
    }

    //修改密码
    pub fn password(&mut self, d: UpdatePassword,username: String) -> Result<(), anyhow::Error> {
        let mut user = self.repo.get_by_username(username.as_str())?;
        // Decrypt the password first
        let decode_base64_old_password = decode(d.old_password)?;
        let decode_base64_new_password = decode(d.new_password)?;
        // 解密密码
        let decoded_old_password = util::encrypt::default_decrypt(&decode_base64_old_password)?;
        let decoded_new_password = util::encrypt::default_decrypt(&decode_base64_new_password)?;
        // 验证密码
        if verify(&decoded_old_password, &user.password)? {
            let hashed_password = hash(decoded_new_password.to_string(), DEFAULT_COST)?;
            user.password = hashed_password;
            self.repo.update(user)?;
            return Ok(());
        }
        bail!("Incorrect password.")
    }
}


#[derive(Debug)]
enum UserStatus {
    #[allow(dead_code)]
    Normal,
    #[allow(dead_code)]
    Disable,
}