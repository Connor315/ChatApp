use actix_web::{web, Responder, HttpResponse};
use sqlx::{Pool, Sqlite};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    chatroom_id: i32,
}

pub async fn chatroom_create(db: web::Data<Pool<Sqlite>>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body("Welcome to our real-time chat application!\n")
}

pub async fn chatroom_join(db: web::Data<Pool<Sqlite>>, info: web::Path<Info>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("{}", info.chatroom_id))
}

// Optional?
pub async fn chatroom_exit(db: web::Data<Pool<Sqlite>>, info: web::Path<Info>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("{}", info.chatroom_id))
}