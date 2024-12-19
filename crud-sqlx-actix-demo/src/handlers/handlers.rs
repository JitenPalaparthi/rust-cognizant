use actix_web::{web, HttpResponse, Responder};
use sqlx::{PgPool};

use crate::models::models::{CreateFood, Food, UpdateFood};

pub async fn create_food(pool: web::Data<PgPool>, food: web::Json<CreateFood>) -> impl Responder {
    let query = "INSERT INTO foods(name,description) VALUES($1,$2) RETURNING *";
    match sqlx::query_as::<_, Food>(query)
        .bind(&food.name)
        .bind(&food.description)
        .fetch_one(pool.get_ref())
        .await
     {
        Ok(new_food) => HttpResponse::Ok().json(new_food),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}

pub async fn update_food(pool: web::Data<PgPool>,id: web::Path<i32>, food: web::Json<UpdateFood>) -> impl Responder {
    let query = "UPDATE foods SET name = COALESCE($1, name), description = COALESCE($2, description), updated_at = EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::BIGINT WHERE id=$3 RETURNING *";
    match sqlx::query_as::<_, Food>(query)
        .bind(&food.name)
        .bind(&food.description)
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await
     {
        Ok(updated_food) => HttpResponse::Ok().json(updated_food),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}


pub async fn get_food(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let query = "SELECT * from foods WHERE id=$1";
    match sqlx::query_as::<_, Food>(query)
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await
     {
        Ok(food) => HttpResponse::Ok().json(food),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}


pub async fn delete_food(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let query = "DELETE from foods WHERE id=$1";
    match sqlx::query(query)
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await
     {
        Ok(_) => HttpResponse::Ok().body("food deleted"),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}


pub async fn get_foods(pool: web::Data<PgPool>) -> impl Responder {
    let query = "SELECT * from foods";
    match sqlx::query_as::<_, Food>(query)
        .fetch_all(pool.get_ref())
        .await
     {
        Ok(foods) => HttpResponse::Ok().json(foods),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    }
}