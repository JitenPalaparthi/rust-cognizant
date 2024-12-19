use actix_web::{self, HttpServer,App,web};
use dotenv::dotenv;
use std::env;
use crate::db::db::create_db_pool;
use handlers::handlers::{create_food,update_food,get_food,get_foods,delete_food};

mod db;
mod models;
mod handlers;

#[actix_web::main]
async fn main()->std::io::Result<()> {
    dotenv().ok();
     let dsn=env::var("DATABASE_URL").expect("database url must be given");
     let pool = create_db_pool(&dsn).await;
println!("app started and running on port 8084");
HttpServer::new(move ||{
    App::new()
    .app_data(web::Data::new(pool.clone()))
    .route("/foods", web::post().to(create_food))
    .route("/foods/{id}", web::get().to(get_food))
    .route("/foods/{id}", web::delete().to(delete_food))
    .route("/foods/{id}", web::put().to(update_food))
    .route("/foods", web::get().to(get_foods))
}).bind("0.0.0.0:8084")?
.run().await
}
