pub struct User {
    conn: mysql::Conn,
}

impl User {

    pub fn new(conn: mysql::Conn) -> Self {
        User { conn }
    }

    pub fn get_user_by_id(&self, id: i32) -> Result<User> {
        // 使用 conn 执行查询并返回 User

    }

}