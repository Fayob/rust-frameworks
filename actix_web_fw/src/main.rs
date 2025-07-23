use actix_web::{App, HttpServer, Responder, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, Arc};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    paswword: String,
    email: String,
}

type Users = Arc<Mutex<Vec<User>>>;

async fn register_user(user: web::Json<User>, users: web::Data<Users>) -> impl Responder {
    let mut users = users.lock().unwrap();
    if users.iter().any(|u| u.username == user.username) {
        return HttpResponse::BadRequest().body("Username already exists");
    }
    users.push(user.into_inner());
    HttpResponse::Created().body("User registered successfully")
}

async fn login_user(user: web::Json<User>, users: web::Data<Users>) -> impl Responder {
    let users = users.lock().unwrap();
    if let Some(found_user) = users.iter().find(|u| u.username == user.username && u.paswword == user.paswword) {
        HttpResponse::Ok().json(found_user)
    } else {
        HttpResponse::BadRequest().body("Invalid username or password")
    }
}

async fn update_account(user: web::Json<User>) -> impl Responder {
    // This is a placeholder for account update logic
    HttpResponse::Ok().body("Account updated successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let users: Users = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(users.clone()))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/account/update", web::put().to(update_account))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}