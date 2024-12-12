use sqlx::{sqlite::{self, SqlitePoolOptions}, Pool, Sqlite, migrate::MigrateDatabase};
use sled;
use sled::Db;
// use chrono::Utc;

use crate::channel::ChatMessage;

// In database.rs
// timestamp 优先 username只储存不排序
pub fn append_chat_message_sled(
    sled_db: &Db,
    channel_name: &str,
    username: &str,
    message: &str,
) -> Result<(), sled::Error> {
    let timestamp = chrono::Local::now()
        .format("%Y-%m-%d %H:%M")
        .to_string();
    
    // Rest of the function remains the same
    let tree = sled_db.open_tree(channel_name)?;
    let key = format!("{}:{}", timestamp, username);
    let value = format!("{}:{}", username, message);
    
    tree.insert(key.as_bytes(), value.as_bytes())?;
    Ok(())
}

pub fn get_chat_history_sled(
    sled_db: &Db,
    channel_name: &str,
) -> Result<Vec<ChatMessage>, sled::Error> {
    println!("=== BEGIN CHAT HISTORY FETCH ===");
    println!("Getting history for channel: {}", channel_name);
    
    let tree = match sled_db.open_tree(channel_name) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let mut messages = Vec::new();
    
    for item in tree.iter() {
        match item {
            Ok((key, value)) => {
                if let (Ok(key_str), Ok(value_str)) = (
                    String::from_utf8(key.to_vec()),
                    String::from_utf8(value.to_vec())
                ) {
                    println!("\nProcessing message:");
                    println!("Key: {}", key_str);
                    println!("Value: {}", value_str);
                    
                    // Split the key into timestamp and username
                    // Format is "YYYY-MM-DD HH:MM:username"
                    if let Some(last_colon_pos) = key_str.rfind(':') {
                        let (timestamp, username) = key_str.split_at(last_colon_pos);
                        let username = &username[1..]; // Remove the leading ':'
                        
                        // Get message from value
                        // Value format is "username:message"
                        if let Some((_, message)) = value_str.split_once(':') {
                            println!("✓ Parsed successfully:");
                            println!("  Timestamp: {}", timestamp);
                            println!("  Username: {}", username);
                            println!("  Message: {}", message);
                            
                            if message != "ping" {
                                let chat_message = ChatMessage {
                                    timestamp: timestamp.to_string(),
                                    username: username.to_string(),
                                    message: message.to_string(),
                                };
                                messages.push(chat_message);
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Error reading message: {}", e),
        }
    }

    messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(messages)
}


pub async fn init_sqlite_db() -> Pool<Sqlite> {
    if !Sqlite::database_exists("sqlite:chat_sqlite.db").await.unwrap_or(false) {
        match Sqlite::create_database("sqlite:chat_sqlite.db").await {
            Ok(_) => {},
            Err(error) => panic!("error: {}", error),
        }
    }

    let init: Result<Pool<Sqlite>, sqlx::Error> = SqlitePoolOptions::new().connect("sqlite:chat_sqlite.db").await;

    let db: Pool<Sqlite>;
    match init {
        Ok(sqlite_db) => {
            println!("Sqlite database initialized successfully.");
            db = sqlite_db;
        },
        Err(e) => {
            panic!("Failed to initialize Sqlite database: {}", e);
        }
    }

    let query: Result<sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("
        CREATE TABLE IF NOT EXISTS Users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            Username TEXT NOT NULL UNIQUE,
            Password TEXT NOT NULL,
            Status TEXT NOT NULL DEFAULT 'offline'
        );

        CREATE TABLE IF NOT EXISTS Channel (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            Name TEXT NOT NULL UNIQUE,
            Owner TEXT NOT NULL,
            FOREIGN KEY (Owner) REFERENCES Users(Username) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_users_username ON Users(Username);
        CREATE INDEX IF NOT EXISTS idx_channel_name ON Channel(Name);").execute(&db).await;

    match query {
        Ok(_) => {
            println!("Tables created successfully");
        },
        Err(e) => {
            panic!("Failed to initialize the SQLite database: {}", e);
        }
    }
    return db;
}

pub async fn init_sled_db() -> sled::Db {
    let db: Result<sled::Db, sled::Error> = sled::open("./chat_sled.db");

    match db {
        Ok(sled_db) => {
            // Ensure the chat history tree exists
            sled_db.open_tree("chat_history").expect("Failed to open chat_history tree");
            println!("Sled database initialized successfully.");
            return sled_db;
        }
        Err(e) => {
            panic!("Failed to initialize Sled database: {}", e);
        }
    }
}

