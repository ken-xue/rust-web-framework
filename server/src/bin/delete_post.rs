use diesel::prelude::*;
use std::env::args;
use server::establish_connection;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{target}%");

    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {num_deleted} posts");
}
