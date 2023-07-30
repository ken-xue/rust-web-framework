use std::ops::{DerefMut};
use diesel::{debug_query, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::expression::AsExpression;
use diesel::mysql::Mysql;
use diesel::result::Error;
use diesel::sql_types::Text;
use crate::database;
use crate::system::user::model::{SysUser, UserWithRoleDept};
use crate::database::schema::sys_user::dsl::*;
use crate::database::schema::sys_user;
use crate::system::dept::model::{SysDept, SysUserOfDept};
use crate::system::role::model::{SysRole, SysUserOfRole};
use crate::system::user::request::PageUser;

pub struct UserRepo {
    conn: database::PoolConnection
}

impl UserRepo {

    pub fn default() -> Self {
        let conn = database::pool();
        UserRepo { conn }
    }

    pub fn new(conn: database::PoolConnection) -> Self {
        UserRepo { conn }
    }

    pub fn get_by_id(&mut self, i: u64) -> Result<SysUser, anyhow::Error> {
        let ret = sys_user
            .filter(id.eq(i))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn get_by_username(&mut self, _username: &str) -> Result<SysUser, anyhow::Error> {
        let ret = sys_user.filter(username.eq(_username))
            .select(SysUser::as_select())
            .first(self.conn.deref_mut())?;
        Ok(ret)
    }

    pub fn update(&mut self,user: SysUser) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::update(sys_user.filter(id.eq(user.id)))
            .set(&user)
            .execute(self.conn.deref_mut()).optional()?;
        Ok(ret)
    }

    pub fn add(&mut self, user: SysUser) -> Result<SysUser, anyhow::Error> {
        diesel::insert_into(sys_user::table).values(&user).execute(self.conn.deref_mut())?;
        Ok(user)
    }

    pub fn delete_by_ids(&mut self, ids: Vec<u64>,) -> Result<Option<usize>, anyhow::Error> {
        let ret = diesel::delete(sys_user.filter(id.eq_any(ids)))
            .execute(self.conn.deref_mut()).optional().map_err(Error::from)?;
        Ok(ret)
    }

    pub fn page(&mut self, page: PageUser) -> Result<(Vec<SysUser>, i64), anyhow::Error> {
        //dsl
        use crate::database::schema::sys_user_of_role::dsl::sys_user_of_role;
        use crate::database::schema::sys_role::dsl::sys_role;
        use crate::database::schema::sys_user_of_dept::dsl::sys_user_of_dept;
        use crate::database::schema::sys_dept::dsl::sys_dept;
        //field
        use crate::database::schema::sys_user_of_role::{role_uuid};
        use crate::database::schema::sys_role::{uuid as ruid};
        use crate::database::schema::sys_user_of_dept::{dept_uuid};
        //query user->role->dept
        let query = sys_user
            //role
            .left_join(sys_user_of_role.on(uuid.eq(database::schema::sys_user_of_role::user_uuid)))
            .left_join(sys_role.on(role_uuid.eq(ruid)))
            //dept
            .left_join(sys_user_of_dept.on(uuid.eq(database::schema::sys_user_of_dept::user_uuid)))
            .left_join(sys_dept.on(dept_uuid.eq(database::schema::sys_dept::uuid)));
        //page query
        let records = query
            .limit(page.page_size).offset(page.page_size * (page.page - 1))
            .select(SysUser::as_select())// actual i want full row data, but when i use `.select(<(SysUser,SysRole,SysDept)>::as_select())` occur error
            .load::<SysUser>(self.conn.deref_mut())?;
        //total
        let total_count = query.count().first::<i64>(self.conn.deref_mut())?;
        let records: Vec<SysUser> = records.into_iter().map(|u| u.into()).collect();
        Ok((records, total_count))
    }
}