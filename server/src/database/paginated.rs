// use diesel::mysql::Mysql;
// use diesel::prelude::*;
// use diesel::query_builder::*;
// use diesel::query_dsl::methods::LoadQuery;
// use diesel::sql_types::BigInt;
//
// pub trait Paginate: Sized {
//     fn paginate(self, page: i64) -> Paginated<Self>;
// }
//
// impl<T> Paginate for T {
//     fn paginate(self, page: i64) -> Paginated<Self> {
//         Paginated {
//             query: self,
//             page_size: DEFAULT_PER_PAGE,
//             page,
//             offset: (page - 1) * DEFAULT_PER_PAGE,
//         }
//     }
// }
//
// const DEFAULT_PER_PAGE: i64 = 10;
//
// #[derive(Debug, Clone, Copy, QueryId)]
// pub struct Paginated<T> {
//     query: T,
//     page: i64,
//     page_size: i64,
//     offset: i64,
// }
//
// impl<T> Paginated<T> {
//     pub fn page_size(self, page_size: i64) -> Self {
//         Paginated {
//             page_size,
//             offset: (self.page - 1) * page_size,
//             ..self
//         }
//     }
//
//     pub fn load_and_count_pages<'a, U>(self, conn: &mut MysqlConnection) -> QueryResult<(Vec<U>, i64)>
//         where
//             Self: LoadQuery<'a, MysqlConnection, (U, i64)>,
//     {
//         let page_size = self.page_size;
//         let results = self.load::<(U, i64)>(conn)?;
//         let total = results.get(0).map(|x| x.1).unwrap_or(0);
//         let records = results.into_iter().map(|x| x.0).collect();
//         let total_pages = (total as f64 / page_size as f64).ceil() as i64;
//         Ok((records, total_pages))
//     }
// }
//
// impl<T: Query> Query for Paginated<T> {
//     type SqlType = (T::SqlType, BigInt);
// }
//
// impl<T> RunQueryDsl<MysqlConnection> for Paginated<T> {}
//
// impl<T> QueryFragment<Mysql> for Paginated<T>
//     where
//         T: QueryFragment<Mysql>,
// {
//     fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Mysql>) -> QueryResult<()> {
//         out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
//         self.query.walk_ast(out.reborrow())?;
//         out.push_sql(") t LIMIT ");
//         out.push_bind_param::<BigInt, _>(&self.page_size)?;
//         out.push_sql(" OFFSET ");
//         out.push_bind_param::<BigInt, _>(&self.offset)?;
//         Ok(())
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use std::ops::DerefMut;
//     use crate::database::{pool};
//     use crate::database::schema::sys_user::dsl::*;
//     use crate::system::user::model::SysUser;
//     use super::*;
//     #[test]
//     fn rsas() {
//         let mut conn = pool();
//         let results: Vec<(SysUser, i64)> = sys_user
//             .paginate(3)
//             .page_size(25)
//             .load(conn.deref_mut())
//             .expect("error");
//     }
// }