use actix_session::Session;
use actix_web::{web, Responder, HttpResponse};
use sqlx::{Pool, Sqlite};
use serde::{Deserialize, Serialize};
use crate::user::check_auth;

#[derive(Deserialize)]
pub struct ChannelRequest {
    name: String,
}

#[derive(Deserialize, sqlx::FromRow, Serialize, Debug)]
struct Channel {
    id: i32,
    name: String,
    owner: String,
}

pub async fn channel_create(db: web::Data<Pool<Sqlite>>, session: Session, info: web::Json<ChannelRequest>) -> impl Responder {
    if !check_auth(&session).is_ok() {
        return HttpResponse::Unauthorized().json("User not logged in.")
    }

    let user_username: String = match session.get::<String>("user_username").unwrap() {
        Some(username) => username,
        None => return HttpResponse::Unauthorized().json("User not logged in.")
    };

    let query: &str = "INSERT INTO Channel (Name, Owner) VALUES (?, ?);";

    let result = sqlx::query(query)
        .bind(&info.name)
        .bind(user_username)
        .execute(db.get_ref())
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

pub async fn channel_list(db: web::Data<Pool<Sqlite>>, session: Session) -> impl Responder {
    if !check_auth(&session).is_ok() {
        return HttpResponse::Unauthorized().json("User not logged in.")
    }

    let query = "SELECT id, Name AS name, Owner AS owner FROM Channel;";

    let result: Result<Vec<Channel>, sqlx::Error> = sqlx::query_as::<_, Channel>(query)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}