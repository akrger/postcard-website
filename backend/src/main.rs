use backend::establish_connection;
use std::env;
use std::path::Path;
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;
use std::time::Duration;
embed_migrations!();

#[macro_use]
extern crate diesel_migrations;

fn main() {

   // let con = establish_connection();

let manager = ConnectionManager::<PgConnection>::new("postgres://postgres@localhost/test");

let pool = r2d2::Pool::builder().build(manager).expect("error");

    embedded_migrations::run(&pool.clone().get().expect("err"));
println!("test");
}
