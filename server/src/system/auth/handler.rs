use std::collections::{HashSet};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use axum::response::IntoResponse;
use crate::{database, system};
use crate::common::error::AppError;
use crate::common::response;
use crate::common::validator::Validated;
use crate::system::auth::{AuthBody, AuthError, AuthPayload, Claims, CURRENT_USER, KEYS};
use crate::system::menu::response::MenuResponse;
use crate::system::{menu, user};

// 登录授权
pub async fn login(Validated(payload): Validated<AuthPayload>) -> Result<impl IntoResponse, AuthError> {
    // Check if the user sent the credentials
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    let mut domain = system::user::service::UserService::default();
    // check
    let user = domain.authorize(payload.username, payload.password).map_err(|e| AuthError::WrongCredentials)?;
    // token
    let current_time = chrono::Utc::now().timestamp();
    let claims = Claims {
        sub: user.username.to_owned(),
        exp: (current_time + 60 * 1000) as usize, // Mandatory expiry time as UTC timestamp
        // exp: (current_time + 10) as usize, // Mandatory expiry time as UTC timestamp
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)?;
    // Send the authorized token
    let body = AuthBody::new(token);
    // distinct all permission
    let mut permissions_set: HashSet<String> = HashSet::new();
    // 遍历user.roles，中的每一个 menus 的 perms字段加入permissions
    if let Some(roles) = &user.roles {
        for role in roles.iter() {
            if let Some(menus) = &role.menus {
                for menu in menus {
                    if let Some(permission) = &menu.permission {
                        let apis = permission.split(",");
                        for key in apis {
                            permissions_set.insert(key.to_string());
                        }
                    }
                }
            }
        }
    }
    let permissions: Vec<&str> = permissions_set.iter().map(|s| s.as_str()).collect();
    // save permission to cached
    database::redis::sadd(user.username, permissions.as_slice()).map_err(|e| AuthError::Unknown(e.to_string()))?;
    // response
    Ok(response::success(body))
}

// 退出登录
pub async fn logout() -> Result<impl IntoResponse, AuthError> {
    let username = CURRENT_USER.with(|cell| {
        cell.borrow().clone()
    });
    //清理缓存
    database::redis::del(username.unwrap()).map_err(|_| AuthError::InvalidToken)?;
    Ok(response::success(""))
}

// 获取当前用户的菜单权限
pub async fn menus() -> Result<impl IntoResponse, AppError> {
    let username = CURRENT_USER.with(|cell| {
        cell.borrow().clone()
    });
    // 查询角色和菜单
    let user = user::service::UserService::default().get_by_username(username.unwrap())?;
    let mut menu_set: HashSet<MenuResponse> = HashSet::new();
    if let Some(roles) = &user.roles {
        for role in roles.iter() {
            if let Some(menus) = &role.menus {
                menu_set.extend(menus.iter().cloned());
            }
        }
    }
    let menus: Vec<MenuResponse> = menu_set.into_iter().collect();
    // 构建成菜单树
    let tree_menus = menu::service::MenuService::default().tree(menus)?;
    // 响应
    Ok(response::success(tree_menus))
}


use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub path: String,
    pub name: String,
    pub component: Option<String>,
    pub meta: Option<MenuMeta>,
    pub redirect: Option<String>,
    pub children: Option<Vec<Menu>>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuMeta {
    pub title: Option<String>,
    pub icon: Option<String>,
    pub hideChildrenInMenu: Option<bool>,
    pub ignoreKeepAlive: Option<bool>,
}


// list 获取当前用户的菜单权限
// pub async fn menus() -> Result<impl IntoResponse, AppError> {
//     let response: Vec<Menu> = serde_json::from_str(JSON_DATA).unwrap();
//     Ok(response::success(response))
// }

const JSON_DATA: &'static str = r#"
[{
  "path": "/dashboard",
  "name": "Dashboard",
  "component": "LAYOUT",
  "redirect": "/dashboard/analysis",
  "meta": {
    "title": "routes.dashboard.dashboard",
    "hideChildrenInMenu": true,
    "icon": "bx:bx-home"
  },
  "children": [
    {
      "path": "analysis",
      "name": "Analysis",
      "component": "/dashboard/analysis/index",
      "meta": {
        "hideMenu": true,
        "hideBreadcrumb": true,
        "title": "routes.dashboard.analysis",
        "currentActiveMenu": "/dashboard",
        "icon": "bx:bx-home"
      }
    },
    {
      "path": "workbench",
      "name": "Workbench",
      "component": "/dashboard/workbench/index",
      "meta": {
        "hideMenu": true,
        "hideBreadcrumb": true,
        "title": "routes.dashboard.workbench",
        "currentActiveMenu": "/dashboard",
        "icon": "bx:bx-home"
      }
    }
  ]
},

{
  "path": "/back",
  "name": "PermissionBackDemo",
  "meta": {
    "title": "routes.demo.permission.back"
  },

  "children": [
    {
      "path": "page",
      "name": "BackAuthPage",
      "component": "/demo/permission/back/index",
      "meta": {
        "title": "routes.demo.permission.backPage"
      }
    },
    {
      "path": "btn",
      "name": "BackAuthBtn",
      "component": "/demo/permission/back/Btn",
      "meta": {
        "title": "routes.demo.permission.backBtn"
      }
    }
  ]
},

{
  "path": "/permission",
  "name": "Permission",
  "component": "LAYOUT",
  "redirect": "/permission/front/page",
  "meta": {
    "icon": "carbon:user-role",
    "title": "routes.demo.permission.permission"
  },
  "children": [
      {
  "path": "back",
  "name": "PermissionBackDemo",
  "meta": {
    "title": "routes.demo.permission.back"
  },

  "children": [
        {
          "path": "page",
          "name": "BackAuthPage",
          "component": "/demo/permission/back/index",
          "meta": {
            "title": "routes.demo.permission.backPage"
          }
        },
        {
          "path": "btn",
          "name": "BackAuthBtn",
          "component": "/demo/permission/back/Btn",
          "meta": {
            "title": "routes.demo.permission.backBtn"
          }
        }
      ]
    }
  ]
},

{
  "path": "/level",
  "name": "Level",
  "component": "LAYOUT",
  "redirect": "/level/menu1/menu1-1",
  "meta": {
    "icon": "carbon:user-role",
    "title": "routes.demo.level.level"
  },

  "children": [
    {
      "path": "menu1",
      "name": "Menu1Demo",
      "meta": {
        "title": "Menu1"
      },
      "children": [
        {
          "path": "menu1-1",
          "name": "Menu11Demo",
          "meta": {
            "title": "Menu1-1"
          },
          "children": [
            {
              "path": "menu1-1-1",
              "name": "Menu111Demo",
              "component": "/demo/level/Menu111",
              "meta": {
                "title": "Menu111"
              }
            }
          ]
        },
        {
          "path": "menu1-2",
          "name": "Menu12Demo",
          "component": "/demo/level/Menu12",
          "meta": {
            "title": "Menu1-2"
          }
        }
      ]
    },
    {
      "path": "menu2",
      "name": "Menu2Demo",
      "component": "/demo/level/Menu2",
      "meta": {
        "title": "Menu2"
      }
    }
  ]
},

{
  "path": "/system",
  "name": "System",
  "component": "LAYOUT",
  "redirect": "/system/account",
  "meta": {
    "icon": "ion:settings-outline",
    "title": "routes.demo.system.moduleName"
  },
  "children": [
    {
      "path": "account",
      "name": "AccountManagement",
      "meta": {
        "title": "routes.demo.system.account",
        "ignoreKeepAlive": true
      },
      "component": "/demo/system/account/index"
    },
    {
      "path": "account_detail/:id",
      "name": "AccountDetail",
      "meta": {
        "hideMenu": true,
        "title": "routes.demo.system.account_detail",
        "ignoreKeepAlive": true,
        "showMenu": false,
        "currentActiveMenu": "/system/account"
      },
      "component": "/demo/system/account/AccountDetail"
    },
    {
      "path": "role",
      "name": "RoleManagement",
      "meta": {
        "title": "routes.demo.system.role",
        "ignoreKeepAlive": true
      },
      "component": "/demo/system/role/index"
    },

    {
      "path": "menu",
      "name": "MenuManagement",
      "meta": {
        "title": "routes.demo.system.menu",
        "ignoreKeepAlive": true
      },
      "component": "/demo/system/menu/index"
    },
    {
      "path": "dept",
      "name": "DeptManagement",
      "meta": {
        "title": "routes.demo.system.dept",
        "ignoreKeepAlive": true
      },
      "component": "/demo/system/dept/index"
    },
    {
      "path": "changePassword",
      "name": "ChangePassword",
      "meta": {
        "title": "routes.demo.system.password",
        "ignoreKeepAlive": true
      },
      "component": "/demo/system/password/index"
    }
  ]
},

 {
  "path": "/link",
  "name": "Link",
  "component": "LAYOUT",
  "meta": {
    "icon": "ion:tv-outline",
    "title": "routes.demo.iframe.frame"
  },
  "children": [
    {
      "path": "doc",
      "name": "Doc",
      "meta": {
        "title": "routes.demo.iframe.doc",
        "frameSrc": "https://doc.vvbin.cn/"
      }
    },
    {
      "path": "https://doc.vvbin.cn/",
      "name": "DocExternal",
      "component": "LAYOUT",
      "meta": {
        "title": "routes.demo.iframe.docExternal"
      }
    }
  ]
}]
"#;