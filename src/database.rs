use sqlx::{sqlite::{self, SqlitePoolOptions}, Pool, Sqlite, migrate::MigrateDatabase};
use sled;

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
            Password TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS Groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            GroupName TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS GroupMembers (
            UserID INTEGER,
            GroupID INTEGER,
            PRIMARY KEY (UserID, GroupID),
            FOREIGN KEY (UserID) REFERENCES Users(id),
            FOREIGN KEY (GroupID) REFERENCES Groups(id)
        );").execute(&db).await;

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
            println!("Sled database initialized successfully.");
            return sled_db;
        },
        Err(e) => {
            panic!("Failed to initialize Sled database: {}", e);
        }
    }
}
