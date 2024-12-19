use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(Mutex::new(0));
    HttpServer::new(move || {
        let counter_data = web::Data::new(counter.clone());
        App::new()
            .app_data(counter_data.clone())
            .route("/", web::get().to(hello))
            .route("/ping", web::get().to(ping))
            .route("/health", web::get().to(health))
    })
    .bind("0.0.0.0:8083")?
    .run()
    .await
}

async fn hello(counter: web::Data<Arc<Mutex<i32>>>) -> impl Responder {
    let mut c = counter.lock().unwrap();
    *c += 1;

    format!("hello World. Counter:{}",c)
}

async fn ping(counter: web::Data<Arc<Mutex<i32>>>) -> impl Responder {
  let mut c = counter.lock().unwrap();
  *c += 1;

  format!("Pong. Counter:{}",c)
}

async fn health(counter: web::Data<Arc<Mutex<i32>>>) -> impl Responder {
  let mut c = counter.lock().unwrap();
  *c += 1;

  format!("Ok.Counter:{}",c)
}
