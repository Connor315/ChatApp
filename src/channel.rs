use actix_session::Session;
use actix_web::{web, Responder, HttpResponse};
use sqlx::{Pool, Sqlite};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChannelRequest {
    name: String,
}

#[derive(Deserialize, sqlx::FromRow)]
pub struct Channel {
    id: i32,
}

pub async fn channel_create(db: web::Data<Pool<Sqlite>>, session: Session, info: web::Json<ChannelRequest>) -> impl Responder {
    let user_id = match session.get::<u32>("user_id").unwrap() {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().json("User not logged in."),
    };

    println!("{}", user_id);

    let query: &str = "INSERT INTO Channel (Name) VALUES (?) RETURNING id";

    let result: Result<Channel, sqlx::Error> = sqlx::query_as::<_, Channel>(query)
        .bind(&info.name)
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(channel) => {
            let channel_id: i32 = channel.id;
            let insert_result = sqlx::query(
                "INSERT INTO ChannelMembers (UserID, ChannelID) VALUES (?, ?)"
            )
                .bind(&user_id)
                .bind(channel_id)
                .execute(db.get_ref())
                .await;
            
            match insert_result {
                Ok(_) => HttpResponse::Ok().json("Channel and membership created successfully!"),
                Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create channel membership: {}", e)),
            }
        },
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create channel: {}", e))
    }
}

// pub async fn channel_join(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
//     // TODO
//     HttpResponse::Ok().body(format!("{}", info.name))
// }

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