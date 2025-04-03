use actix_web::{HttpServer, App, web};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::controller;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("The DATABASE_URL variable is not set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(controller::register))
            .route("/login", web::post().to(controller::login))
            .route("/protected", web::get().to(controller::protected))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
