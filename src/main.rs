use actix_web::{web, App, HttpServer, Responder, HttpResponse, cookie::Key};
use sqlx::{Pool, Sqlite};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use sled::Db;
use tokio;

mod database;
mod user;
mod chatroom;
mod status;
mod ui;
mod websocket;

use database::init_sqlite_db;
use database::init_sled_db;
use user::register;
use user::login;
use user::logout;

async fn welcome() -> impl Responder {
    // TODO: Add Help menu
    HttpResponse::Ok().body("Welcome to our real-time chat application!\n")
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let sqlite_db: Pool<Sqlite> = init_sqlite_db().await;
    let sled_db: Db = init_sled_db().await;
    let secret_key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(sqlite_db.clone()))
            .app_data(web::Data::new(sled_db.clone()))
            // .route("/count", web::get().to(count))
            .route("/", web::get().to(welcome))
            .service(
                web::scope("/user")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
                    .route("/logout", web::post().to(logout))
            )
    })
    .bind("127.0.0.1:8080")?;

    println!("The server is currently listening on localhost:8080.");
    server.run().await
}
