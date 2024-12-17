use std::fmt::format;

use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

use serde_derive::{Serialize,Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server started and listening on port:8087");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/ping", web::get().to(ping))
            .route("/health", web::get().to(health))
            .route("/user", web::post().to(create_user))
    })
    .bind("0.0.0.0:8087")?
    .run()
    .await
}



async fn ping() -> impl Responder {
    "pong"
}

async fn health() -> impl Responder {
    "ok"
}


#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

impl HelloResponse {
    fn new(msg: String) -> Self {
        Self { message: msg }
    }
}

async fn hello() -> impl Responder {
    let obj = HelloResponse::new("Hello World!".to_string());
    web::Json(obj)
}
#[derive(Serialize,Deserialize)]
struct CreateUser {
    name: String,
    age: i8,
}

impl CreateUser {
    fn new(name: String, age: i8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }
}

async fn create_user(user: web::Json<CreateUser>) -> impl Responder {
    format!("User Details. Name:{} Age:{}", user.name, user.age)
}
