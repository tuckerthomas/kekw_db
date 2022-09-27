#[macro_use]
extern crate diesel;
extern crate diesel_migrations;

use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{
    ConnectionManager,
    Pool
};

pub mod periods;
pub mod submissions;
pub mod rolls;
pub mod models;
pub mod schema;

pub type KekPool = Pool::<ConnectionManager::<PgConnection>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn establish_connection() -> KekPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set!");

    Pool::builder().build(ConnectionManager::<PgConnection>::new(database_url)).expect("Could not creat pool")
}
