use serde::{Deserialize};
use validator::{Validate};
use crate::system::user::model::SysUser;
use crate::system::user::response::UserResponse;
use crate::util;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageUser {
    pub page: i64,
    pub page_size: i64,
    pub username: Option<String>,//账号
    pub name: Option<String>,//名字
    pub email:Option< String>,//邮箱
    pub status: Option<i32>,
}

impl From<AddUser> for SysUser {
    fn from(user: AddUser) -> SysUser {
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
