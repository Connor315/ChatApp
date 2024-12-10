use actix_session::Session;
use actix_web::{web, Responder, HttpResponse};
use sqlx::{Pool, Sqlite};
use serde::Deserialize;
use std::sync::Arc;
use crate::user::check_auth;
use actix_web::HttpRequest;
use crate::websocket::ChatState;
use actix_web_actors::ws;
use crate::database::get_chat_history_sled;
use crate::websocket::ChatSession;



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

// pub async fn channel_enter(
//     db: web::Data<Pool<Sqlite>>,
//     sled_db: web::Data<sled::Db>,
//     info: web::Path<ChannelRequest>,
//     state: web::Data<Arc<ChatState>>,
//     session: Session,
//     req: HttpRequest,
//     stream: web::Payload,
// ) -> impl Responder {
//     let channel_name = &info.name;

//     let (_user_id, user_name) = match check_auth(&session) {
//         Ok((id, username)) => (id, username),
//         Err(err) => return HttpResponse::Unauthorized().body(err.to_string()),
//     };

//     // Check if the channel exists
//     match sqlx::query!("SELECT * FROM Channel WHERE Name = ?", channel_name)
//         .fetch_optional(db.get_ref())
//         .await
//     {
//         Ok(Some(_)) => {
//             let resp = ws::start(
//                 ChatSession::new(
//                     user_name,         // Pass user_name dynamically
//                     channel_name.to_string(),       // Pass channel_name dynamically
//                     state.get_ref().clone(),        // Pass the shared state
//                     sled_db.clone(),                // Pass the Sled database
//                 ),
//                 &req,
//                 stream,
//             );
//             match resp {
//                 Ok(response) => response,
//                 Err(_) => HttpResponse::InternalServerError().finish(),
//             }
//         }
//         Ok(None) => HttpResponse::NotFound().json("Channel not found."),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }

pub async fn channel_enter() -> impl Responder {
    println!("channel_enter invoked with channel_name");

    HttpResponse::Ok().body("Channel enter endpoint works!")
}



// Optional?
// pub async fn channel_exit(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
//     // TODO
//     HttpResponse::Ok().body(format!("{}", info.name))
// }

pub async fn channel_history(
    sled_db: web::Data<sled::Db>,
    info: web::Path<ChannelRequest>,
) -> impl Responder {
    let channel_name = &info.name;

    match get_chat_history_sled(&sled_db, channel_name) {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve chat history"),
    }
}

pub async fn channel_list(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
    // TODO
    HttpResponse::Ok().body(format!("{}", info.name))
}