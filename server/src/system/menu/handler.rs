use axum::extract::{Path};
use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{database};
use crate::common::error::AppError;
use crate::common::{request, response};
use crate::system::menu::{service, repo, abc};
use validator::{Validate};
use crate::common::validator::Validated;
use crate::system::menu::request::{CreateMenu, UpdateMenu};
use crate::system::menu::response::Menu;

// info
pub async fn info(Path(id): Path<u64>) -> Result<impl IntoResponse, AppError> {
    let response = service::MenuService::default().get_by_id(id)?;
    Ok(response::success(response))
}
// page
pub async fn page(Json(r): Json<request::Page>) -> Result<impl IntoResponse, AppError> {
    let response = service::MenuService::default().page(r)?;
    Ok(response::success(response))
}
// create
pub async fn create(Validated(r): Validated<CreateMenu>) -> Result<impl IntoResponse, AppError>  {
    let response = service::MenuService::default().create(r)?;
    Ok(response::success(response))
}
// update
pub async fn update(Json(r): Json<UpdateMenu>) -> Result<impl IntoResponse, AppError>  {
    let response = service::MenuService::default().update(r)?;
    Ok(response::success(response))
}
// delete
pub async fn delete(Json(r): Json<request::Delete>)  -> Result<impl IntoResponse, AppError>  {
    let response = service::MenuService::default().delete(r)?;
    Ok(response::success(response))
}

// list 获取当前用户的菜单权限
pub async fn list() -> Result<impl IntoResponse, AppError> {
    // let response = service::MenuService::default().list()?;
    // Ok(response::success(response))
    let response: Vec<Menu> = serde_json::from_str(JSON_DATA).unwrap();
    // let json_value = json!(JSON_DATA);
    // let response = abc::get();
    Ok(response::success(response))
}

#[cfg(test)]
mod tests {
    use redis::RedisError;
    use super::*;
    #[test]
    fn rsas() {
        let response: Vec<Menu> = serde_json::from_str(JSON_DATA).unwrap();
        println!("{}","done")
    }
}

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