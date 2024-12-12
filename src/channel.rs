use actix_session::Session;
use actix_web::{web, Responder, HttpResponse};
use sqlx::{Pool, Sqlite};
use serde::{Deserialize, Serialize};
use crate::user::check_auth;
use crate::database::get_chat_history_sled;
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub timestamp: String,
    pub username: String,
    pub message: String,
}

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

#[derive(Deserialize)]
pub struct ChannelPath {
    name: String,
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

// pub async fn channel_enter(
//     db: web::Data<Pool<Sqlite>>,
//     sled_db: web::Data<sled::Db>,
//     info: web::Path<ChannelRequest>,
//     state: web::Data<Arc<ChatState>>,
//     session: Session,
//     req: HttpRequest,
//     stream: web::Payload,
// ) -> impl Responder {
//     println!("channel_enter called with info: {:?}", info.name);

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
pub async fn channel_enter(
    db: web::Data<Pool<Sqlite>>,
    info: web::Path<ChannelRequest>,
    session: Session,
) -> impl Responder {
    
    if !check_auth(&session).is_ok() {
        return HttpResponse::Unauthorized().json("User not logged in.");
    }

    let channel_name = &info.name;

    // Check if channel exists
    match sqlx::query!("SELECT * FROM Channel WHERE Name = ?", channel_name)
        .fetch_optional(db.get_ref())
        .await
    {
        Ok(Some(_)) => {
            // Return success with WebSocket connection details
            HttpResponse::Ok().json(json!({
                "status": "success",
                "channel": channel_name,
                "ws_url": format!("/ws/{}", channel_name)
            }))
        }
        Ok(None) => {
            println!("Channel not found.");
            HttpResponse::NotFound().json("Channel not found.")
        }
        Err(e) => {
            println!("Error querying database: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}






// Optional?
// pub async fn channel_exit(db: web::Data<Pool<Sqlite>>, info: web::Path<ChannelRequest>) -> impl Responder {
//     // TODO
//     HttpResponse::Ok().body(format!("{}", info.name))
// }

pub async fn channel_history(
    sled_db: web::Data<sled::Db>,
    info: web::Path<ChannelPath>,
) -> impl Responder {
    println!("Accessing channel history for: {}", info.name);
    
    let channel_name = &info.name;
    
    match get_chat_history_sled(&sled_db, channel_name) {
        Ok(messages) => {
            println!("Found {} messages", messages.len());
            // Print each message individually
            for msg in &messages {
                println!("Message - User: {}, Content: {}, Time: {}", 
                    msg.username, msg.message, msg.timestamp);
            }
            HttpResponse::Ok().json(messages)
        },
        Err(err) => {
            println!("Error getting chat history: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve chat history")
        }
    }
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