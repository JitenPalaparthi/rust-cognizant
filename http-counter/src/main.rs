use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(Mutex::new(0));

    HttpServer::new(move || {
        let counter1 = counter.clone();
        let counter2 = counter.clone();
        let counter3 = counter.clone();

        App::new()
            .route("/", web::get().to(move || hello(counter1.clone())))
            .route("/ping", web::get().to(move || ping(counter2.clone())))
            .route("/health", web::get().to(move || health(counter3.clone())))
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}

async fn hello(counter: Arc<Mutex<i32>>) -> impl Responder {
    let mut c = counter.lock().unwrap();
    *c += 1;

    format!("hello World. Counter:{}",c)
}

async fn ping(counter: Arc<Mutex<i32>>) -> impl Responder {
  let mut c = counter.lock().unwrap();
  *c += 1;

  format!("Pong. Counter:{}",c)
}

async fn health(counter: Arc<Mutex<i32>>) -> impl Responder {
  let mut c = counter.lock().unwrap();
  *c += 1;

  format!("Ok.Counter:{}",c)
}
