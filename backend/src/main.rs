#[macro_use]
extern crate rocket;
use rocket::State;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug)]
struct Posts {
    text: Option<String>,
    id: i32,
    author_id: i32,
}
#[derive(sqlx::FromRow, Debug)]
struct PostsWithAuthor {
    text: Option<String>,
    post_id: Option<i32>,
    name: Option<String>,
    author_id: Option<i32>,
}

#[get("/<name>/<age>")]
async fn hello(name: String, age: u8, db: State<'_, PgPool>) -> String {
    let row = sqlx::query_as!(
        PostsWithAuthor,
        "\
    select *  from cool.posts join cool.author using (author_id)\
    "
    )
    .fetch_all(&*db)
    .await;
    // .fetch_one(&*db).await.expect("test2");
    println!("{:?}", row);
    format!("Hello, {} year named {}!", age, name)
}

// #[launch]
// fn rocket() -> rocket::Rocket {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:password@localhost/test").await?;
//     rocket::ignite().mount("/hello", routes![hello])
// }

#[rocket::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await
        .expect("test1");
    rocket::ignite()
        .manage(pool)
        .mount("/hello", routes![hello])
        .launch()
        .await;
}
