// use crate::database::schema::sys_role_of_menu::role_uuid;
// let connection = database::establish_connection();
// let mut conn = LoggingConnection::new(connection);
// let menus = sys_role_of_menu
//     .filter(role_uuid.eq_any(ids))
//     .inner_join(sys_menu.on(uuid.eq(menu_uuid)))
//     .select((role_uuid, SysMenu::as_select()))//需要将role_id也带出来
//     .load::<(String, SysMenu)>(&mut conn)?;
// // Ok(menus)
// let ret: Vec<(String,SysMenu)> = Vec::new();
// Ok(ret)