use actix_session::Session;
use actix_web::{web, Responder, HttpResponse};
use sqlx::{Pool, Sqlite};
use serde::Deserialize;

use crate::user::check_auth;

#[derive(Deserialize)]
pub struct ChannelRequest {
    name: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct Channel {
    id: i32,
}

pub async fn channel_create(db: web::Data<Pool<Sqlite>>, session: Session, info: web::Json<ChannelRequest>) -> impl Responder {
    if !check_auth(&session).is_ok() {
        return HttpResponse::Unauthorized().json("User not logged in.")
    }

    let query: &str = "INSERT INTO Channel (Name) VALUES (?) RETURNING id";

    let result: Result<Channel, sqlx::Error> = sqlx::query_as::<_, Channel>(query)
        .bind(&info.name)
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Channel created successfully!"),
        Err(_) => HttpResponse::Conflict().json("Channel name already exists!")
    }
}

pub async fn channel_enter(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("{}", info.name))
}

// Optional?
// pub async fn channel_exit(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
//     // TODO
//     HttpResponse::Ok().body(format!("{}", info.name))
// }

pub async fn channel_history(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("{}", info.name))
}

pub async fn channel_list(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("{}", info.name))
}