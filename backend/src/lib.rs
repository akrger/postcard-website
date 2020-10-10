#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::path::Path;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    //dotenv().ok();
    //  let database_url = env::var("DATABASE_URL")
    //.expect("DATABASE_URL must be set");
    //println!("{:?}", database_url);
    let con = PgConnection::establish("postgres://postgres@localhost/test").expect("test");

    con
}

pub fn get_all_cards(
    conn: &PgConnection,
) -> std::result::Result<std::vec::Vec<models::Card>, diesel::result::Error> {
    use self::models::*;
    use self::schema::cards::dsl::*;
    let results = cards.load::<Card>(conn).expect("Error");
    Ok(results)
}
