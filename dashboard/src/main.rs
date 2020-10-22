use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use diesel::pg::PgConnection;

use actix_files::Files;
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer, Result};
use tera::Tera;

// store tera template in application state
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("index.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        let manager = ConnectionManager::<PgConnection>::new("postgres://postgres@localhost/test");
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        App::new()
            .data(tera)
            .data(pool.clone())
            .wrap(middleware::Logger::default()) // enable logger
            .service(
                web::scope("/api/")
                    .service(web::resource("/user/").route(web::get().to(get_cards))),
            )
            .service(
                Files::new("/locales/de", "./packages/dashboard/public/locales/de/")
                    .index_file("translation.json"),
            )
            .service(
                Files::new("/locales/en", "./packages/dashboard/public/locales/en/")
                    .index_file("translation.json"),
            )
            .service(web::resource("*").route(web::get().to(index)))

        //   .service(web::scope("").wrap(error_handlers()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_cards(
    pool: web::Data<DbPool>,
    // user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let cards = web::block(move || backend::get_all_cards(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    Ok(HttpResponse::Ok().json(Some(cards)))
    // if let cards = cards {
    //     Ok(HttpResponse::Ok().json(cards))
    // } else {
    //     let res = HttpResponse::NotFound().body("");
    //     Ok(res)
    // }
}
