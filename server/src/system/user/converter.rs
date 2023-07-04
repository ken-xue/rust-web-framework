use crate::system::user::model::SysUser;
use crate::system::user::request::{CreateUser, UpdateUser};
use crate::system::user::response::UserResponse;
use crate::util;

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

impl From<SysUser> for UserResponse {
    fn from(user: SysUser) -> UserResponse {
        UserResponse {
            id: user.id,
            uuid: user.uuid,
            username: user.username,
            name: user.name,
            email: user.email,
            status: user.status,
            creator: user.creator,
            modifier: user.modifier,
            gmt_create: user.gmt_create,
            gmt_modified: user.gmt_modified,
            avatar: user.avatar,
        }
    }
}