use serde::{Deserialize};
use validator::{Validate};

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

// #[derive(Debug, Validate, Deserialize)]
// pub struct UpdateUser {
//     pub id: i64,//主键
//     pub uuid: String,//唯一标示做关联
//     pub username: String,//账号
//     pub password: String,//密码
//     pub name: String,//名字
//     pub email: String,//邮箱
//     pub status: i32,//状态
//     pub avatar: Option<String>,//头像
// }
//
// #[derive(Debug, Validate, Deserialize)]
// pub struct CreateUser {
//     //#[validate(length(min = 1, message = "Can not be empty"))]
//     pub username: String,//账号
//     pub password: String,//密码
//     pub name: String,//名字
//     pub email: String,//邮箱
//     pub status: i32,//状态
//     pub avatar: Option<String>,//头像
// }