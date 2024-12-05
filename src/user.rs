use actix_web::{web, Responder, HttpResponse, Error};
use sqlx::{Pool, Sqlite};
use actix_session::Session;
use serde::Deserialize;
use actix_web::error::ErrorUnauthorized;
use pwhash::bcrypt;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub fn check_auth(session: &Session) -> Result<(u32, String), Error> {
    let user_id = match session.get::<u32>("user_id")? {
        Some(id) => id,
        None => return Err(ErrorUnauthorized("User ID not found in session")),
    };

    let user_username = match session.get::<String>("user_username")? {
        Some(username) => username,
        None => return Err(ErrorUnauthorized("Username not found in session")),
    };

    Ok((user_id, user_username))
}

pub async fn register(db: web::Data<Pool<Sqlite>>, form: web::Json<RegisterRequest>) -> impl Responder {
    let hashed_password: String = bcrypt::hash(&form.password).unwrap();

    let result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> = sqlx::query("INSERT INTO Users (username, password) VALUES (?, ?)")
        .bind(&form.username)
        .bind(&hashed_password)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User registered successfully."),
        Err(_) => HttpResponse::Conflict().json("Username already exists.")
    }
}

pub async fn login(db: web::Data<Pool<Sqlite>>, session: Session, form: web::Json<LoginRequest>) -> impl Responder {
    if check_auth(&session).is_ok() {
        return HttpResponse::BadRequest().body("Logout required before logging in.");
    }

    let user_data: Result<Option<(u32, String)>, sqlx::Error> = sqlx::query_as::<_, (u32, String)>("SELECT id, Password FROM Users WHERE Username = ?")
        .bind(&form.username)
        .fetch_optional(db.get_ref())
        .await;

    let user = match user_data
    {
        Ok(Some(row)) => row,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid username or password."),
        Ok(None) => return HttpResponse::Unauthorized().json("Invalid username or password.")
    };

    if bcrypt::verify(&form.password, &user.1) {
        let id_set = session.insert("user_id", &user.0);
        let username_set = session.insert("user_username", &form.username);

        if id_set.is_ok() && username_set.is_ok() {
            session.renew();
            HttpResponse::Ok().json(format!("Login successful, {}", &form.username))
        } else {
            HttpResponse::InternalServerError().body("Error setting session data")
        }
    } else {
        HttpResponse::Unauthorized().json("Invalid username or password.")
    }
}

pub async fn logout(session: Session) -> impl Responder {
    // println!("{:?}", session.entries());
    if check_auth(&session).is_err() {
        return HttpResponse::NotFound().body("No user logged in.");
    }
    session.clear();
    HttpResponse::Ok().json("Logout successful")
}