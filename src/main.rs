use actix_web::{web, App, HttpServer, cookie::Key, Responder};
use sqlx::{Pool, Sqlite};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use tokio;
use actix_files as fs;

mod database;
mod user;
mod channel;
mod status;
mod websocket;

use database::init_sqlite_db;
use database::init_sled_db;
use user::register;
use user::login;
use user::logout;
use channel::channel_create;
use channel::channel_enter;
// use channel::channel_exit;
use channel::channel_history;
use channel::channel_list;

use std::sync::{Arc, Mutex};
use crate::websocket::ChatState;

// pub async fn auth(session: Session) -> impl Responder {
//     match check_auth(&session) {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
//     }
// }

async fn login_page() -> impl Responder {
    fs::NamedFile::open("./static/login.html")
}

async fn register_page() -> impl Responder {
    fs::NamedFile::open("./static/register.html")
}

async fn channel_list_page() -> impl Responder {
    fs::NamedFile::open("./static/channel_list.html")
}

async fn channel_create_page() -> impl Responder {
    fs::NamedFile::open("./static/channel_create.html")
}

async fn channel_page() -> impl Responder {
    fs::NamedFile::open("./static/channel.html")
}

async fn index() -> impl Responder {
    fs::NamedFile::open("./static/index.html")
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let sqlite_db: Pool<Sqlite> = init_sqlite_db().await;
    // let sled_db: Db = init_sled_db().await;
    let sled_db = web::Data::new(init_sled_db().await);
    let secret_key = Key::generate();
    let chat_state = web::Data::new(Arc::new(ChatState {
        messages: Mutex::new(Vec::new()),
        connected_users: Mutex::new(Vec::new()),
    }));
    
    

    let server = HttpServer::new(move || {
        let chat_state = chat_state.clone();
        let sled_db = sled_db.clone();
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(sqlite_db.clone()))
            .app_data(web::Data::new(sled_db.clone()))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login_page))
            .route("/register", web::get().to(register_page))
            .route("/channel_list", web::get().to(channel_list_page))
            .route("/channel_create", web::get().to(channel_create_page))
            .route("/channel_room", web::get().to(channel_page))
            .service(
                web::scope("/user")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
                    .route("/logout", web::post().to(logout))
            )
            .service(
                web::scope("/channel")
                    .route("/create", web::post().to(channel_create))
                    .route("/list", web::get().to(channel_list))
                    .route("/enter/{name}", web::post().to(channel_enter))
                    // .route("/channel/enter/{name}", web::post().to(|| async {
                    //     println!("Channel enter route hit");
                    //     // HttpResponse::Ok().body("Channel enter route works")
                    // }))
                    // .route("/exit/{channel_name}", web::post().to(channel_exit))
                    .route("/history/{channel_name}", web::post().to(channel_history)))
                    .route("/ws/", web::get().to({
                        let chat_state = chat_state.clone();
                        let sled_db = sled_db.clone();
                        move |req, stream| {
                            websocket::chat_route(req, stream, chat_state.clone(), sled_db.clone())
                        }
                    }))              
            .service(fs::Files::new("/", "./static"))
    })
    .bind("127.0.0.1:8080")?;



    println!("The server is currently listening on localhost:8080.");
    server.run().await
}
