use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::database::append_chat_message_sled;

/// Define interval for ping messages
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Shared state for storing messages and managing connected users
pub struct ChatState {
    pub messages: Mutex<Vec<(String, String, String)>>, // (timestamp, user, message)
    pub connected_users: Mutex<Vec<String>>,           // List of connected users
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
    fn broadcast_message(&self, message: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let users = self.state.connected_users.lock().unwrap().clone();
        for user in users {
            ctx.text(format!("{}: {}", self.user_name, message));
        }
    }
}

/// WebSocket message handler implementation for `ChatSession`
impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        // Add the user to the connected users list
        {
            let mut users = self.state.connected_users.lock().unwrap();
            users.push(self.user_name.clone());
        }

        // Broadcast join message
        let join_message = format!("{} joined the chat", self.user_name);
        self.broadcast_message(&join_message, ctx);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        // Remove the user from the connected users list
        {
            let mut users = self.state.connected_users.lock().unwrap();
            users.retain(|user| user != &self.user_name);
        }

        // Broadcast quit message
        let quit_message = format!("{} left the chat", self.user_name);
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
                    "example_channel", // Replace with dynamic channel name
                    &self.user_name,
                    &text,
                ) {
                    eprintln!("Failed to store chat message in Sled: {}", err);
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
) -> Result<HttpResponse, Error> {
    let channel_name = req.match_info().get("channel").unwrap_or("default").to_string(); // Example dynamic channel
    let resp = ws::start(
        ChatSession::new("User".to_string(), channel_name, state.get_ref().clone(), sled_db.clone()),
        &req,
        stream,
    );
    resp
}




