use serde::{Serialize};
use crate::system::user::model::SysUser;

#[derive(Debug,Serialize)]
pub struct UserResponse {
    pub id: u64,//主键
    pub uuid: String,//唯一标示做关联
    pub username: String,//账号
    pub name: String,//名字
    pub email: String,//邮箱
    pub status: i32,//状态
    pub creator: Option<String>,//创建人
    pub modifier: Option<String>,//修改人
    pub gmt_create: chrono::NaiveDateTime,//创建时间
    pub gmt_modified: chrono::NaiveDateTime,//修改时间
    pub avatar: Option<String>,//头像
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