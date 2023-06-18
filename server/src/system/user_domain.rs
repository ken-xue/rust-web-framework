use std::fmt::{Error};
use std::ops::{DerefMut};

use crate::database;
use crate::system::sys_model::SysUser;
use crate::system::user_handler::CreateUser;
use crate::system::user_repo::UserRepo;

pub struct UserDomain {
    repo: UserRepo,
}

impl UserDomain {
    
    pub fn new(repo: UserRepo) -> Self {
        UserDomain { repo }
    }

    pub fn get_user_by_id(&mut self, i: u64) -> Result<SysUser, Error> {
        self.repo.get_user_by_id(i)
    }

    pub fn update_user(&mut self) {
        use diesel::prelude::*;
        use crate::models::Post;
        use crate::schema::posts::dsl::*;
        let mut connection = database::pool(); // Get a connection from the pool
        let conn = connection.deref_mut(); // Convert the connection to a mutable reference

        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .select(Post::as_select())
            .load::<Post>(conn) // Pass the mutable reference to the connection
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
    }

    pub fn create_user(&mut self, u: CreateUser) -> Result<SysUser, Error> {
        self.repo.create_user(u)
    }
}