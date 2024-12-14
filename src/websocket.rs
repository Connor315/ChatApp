use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use actix_session::SessionExt;
use crate::Sqlite;
use crate::Pool;
use crate::user;
use crate::database::append_chat_message_sled;
use crate::database::append_user_status_sled;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
struct ChatMessage {
    msg: String,
}

// Add Handler implementation for ChatSession
impl Handler<ChatMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.msg);
    }
}

/// Define interval for ping messages
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Shared state for storing messages and managing connected users
// pub struct ChatState {
//     pub messages: Mutex<Vec<(String, String, String)>>, // (timestamp, user, message)
//     pub connected_users: Mutex<Vec<String>>,           // List of connected users
//     pub sessions: Mutex<Vec<Addr<ChatSession>>>,
// }

pub struct ChatState {
    pub messages: Mutex<Vec<(String, String, String)>>, // (timestamp, user, message)
    // pub connected_users: Mutex<Vec<String>>,           // List of connected users
    pub sessions: Mutex<HashMap<String, Vec<Addr<ChatSession>>>>, // Map of channel to sessions
}

/// Define the WebSocket connection structure
pub struct ChatSession {
    hb: Instant,              // Client's last heartbeat
    user_name: String,        // Name of the user
    channel_name: String,     // Channel name
    state: Arc<ChatState>,    // Shared state across sessions
    sled_db: web::Data<sled::Db>, // Sled database instance
}


impl ChatSession {
    /// Create a new instance of the chat session
    pub fn new(
        user_name: String,
        channel_name: String,
        state: Arc<ChatState>,
        sled_db: web::Data<sled::Db>,
    ) -> Self {
        Self {
            hb: Instant::now(),
            user_name,
            channel_name,
            state,
            sled_db,
        }
    }

    /// Handle the WebSocket heartbeat
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // Check the duration of the last heartbeat from the client
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Client heartbeat failed, disconnecting");
                ctx.stop();
                return;
            }

            // Send a ping message to the client
            ctx.ping(b"");
        });
    }

    /// Broadcast a message to all connected clients
    // fn broadcast_message(&self, message: &str, _ctx: &mut ws::WebsocketContext<Self>) {
    //     // Format the message
    //     let msg = format!("{}: {}", self.user_name, message);
        
    //     // Get all sessions and broadcast to them
    //     if let Ok(sessions) = self.state.sessions.lock() {
    //         for session in sessions.iter() {
    //             session.do_send(ChatMessage { msg: msg.clone() });
    //         }
    //     }
    // }

    fn broadcast_message(&self, message: &str, _ctx: &mut ws::WebsocketContext<Self>) {
        let msg = format!("{}: {}", self.user_name.trim(), message.trim());
        
        // Get sessions for the current channel
        if let Ok(sessions_map) = self.state.sessions.lock() {
            if let Some(sessions) = sessions_map.get(&self.channel_name) {
                for session in sessions {
                    session.do_send(ChatMessage { msg: msg.clone() });

                    println!("Broadcasting message: {}", msg);
                }
            }
        }
    }
    
    
    
}

/// WebSocket message handler implementation for `ChatSession`
impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    // fn started(&mut self, ctx: &mut Self::Context) {
    //     self.hb(ctx);
    
    //     // Store session address
    //     if let Ok(mut sessions) = self.state.sessions.lock() {
    //         sessions.push(ctx.address());
    //     }
    
    //     // Add the user to the connected users list
    //     {
    //         let mut users = self.state.connected_users.lock().unwrap();
    //         users.push(self.user_name.clone());
    //     }
    
    //     // Broadcast join message
    //     let join_message = format!("{} joined the chat", self.user_name);
    //     self.broadcast_message(&join_message, ctx);
    // }

    // fn stopped(&mut self, ctx: &mut Self::Context) {
    //     // Remove session
    //     if let Ok(mut sessions) = self.state.sessions.lock() {
    //         sessions.retain(|x| x != &ctx.address());
    //     }
    
    //     // Remove the user from the connected users list
    //     {
    //         let mut users = self.state.connected_users.lock().unwrap();
    //         users.retain(|user| user != &self.user_name);
    //     }
    
    //     // Broadcast quit message
    //     let quit_message = format!("{} left the chat", self.user_name);
    //     self.broadcast_message(&quit_message, ctx);
    // }
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    
        if let Ok(mut sessions_map) = self.state.sessions.lock() {
            sessions_map
                .entry(self.channel_name.clone())
                .or_insert_with(Vec::new)
                .push(ctx.address());
        }
    
        let join_message = format!("{} joined the chat", self.user_name);
        if let Err(err) = append_user_status_sled(
            &self.sled_db,
            &self.channel_name, 
            &self.user_name,
            true
        ) {
            println!("Failed to store user status in Sled: {}", err);
        }
        self.broadcast_message(&join_message, ctx);
    }
    
    fn stopped(&mut self, ctx: &mut Self::Context) {
        if let Ok(mut sessions_map) = self.state.sessions.lock() {
            if let Some(sessions) = sessions_map.get_mut(&self.channel_name) {
                sessions.retain(|addr| addr != &ctx.address());
                if sessions.is_empty() {
                    sessions_map.remove(&self.channel_name);
                }
            }
        }
    
        let quit_message = format!("{} left the chat", self.user_name);
        if let Err(err) = append_user_status_sled(
            &self.sled_db,
            &self.channel_name, 
            &self.user_name,
            false
        ) {
            println!("Failed to store user status in Sled: {}", err);
        }
        self.broadcast_message(&quit_message, ctx);
    }
    
}

/// Implement `StreamHandler` to handle the incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let timestamp = Utc::now().to_rfc3339();

                // Store the message in the shared state
                {
                    let mut messages = self.state.messages.lock().unwrap();
                    messages.push((timestamp.clone(), self.user_name.clone(), text.to_string()));
                }

                // Append the message to the Sled database using `self.sled_db`
                if let Err(err) = append_chat_message_sled(
                    &self.sled_db, // Access sled_db from the struct
                    &self.channel_name, 
                    &self.user_name,
                    &text,
                ) {
                    println!("Failed to store chat message in Sled: {}", err);
                }

                // Broadcast the message to all clients
                self.broadcast_message(&text, ctx);
            }

            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}


/// WebSocket handler function
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Arc<ChatState>>,
    sled_db: web::Data<sled::Db>,
    channel_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    println!("WebSocket connection attempt for channel: {}", channel_name);
    
    // Get session from request
    let session = req.get_session();
    
    // Check authentication
    let (_user_id, username) = match user::check_auth(&session) {
        Ok((id, name)) => (id, name),
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    // Get database connection from app data
    let db = match req.app_data::<web::Data<Pool<Sqlite>>>() {
        Some(db) => db,
        None => return Ok(HttpResponse::InternalServerError().finish()),
    };

    // Get database reference first
    let db_ref = db.get_ref();

    // Execute query and handle the result
    let channel_name_ref = channel_name.as_ref();
    let query = sqlx::query("SELECT * FROM Channel WHERE Name = ?").bind(channel_name_ref);
    let channel_exists = query.fetch_optional(db_ref).await.map_err(|e| {
        println!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // Check if channel exists
    if channel_exists.is_none() {
        println!("Channel not found: {}", channel_name);
        return Ok(HttpResponse::NotFound().finish());
    }

    // Start WebSocket connection
    println!("Starting WebSocket connection for user {} in channel {}", username, channel_name);
    
    let resp = ws::start(
        ChatSession::new(
            username,
            channel_name.to_string(),
            state.get_ref().clone(),
            sled_db.clone(),
        ),
        &req,
        stream,
    )?;

    println!("WebSocket connection established");
    Ok(resp)
}
