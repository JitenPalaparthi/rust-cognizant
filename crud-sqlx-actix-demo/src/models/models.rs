use serde::{Serialize,Deserialize};
use sqlx::FromRow;

#[derive(Serialize,Deserialize,FromRow)]
pub struct Food{
    pub id:i32,
    pub name:String,
    pub description:Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Deserialize)]
pub struct CreateFood{
    pub name:String,
    pub description:Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateFood{
    pub name:Option<String>,
    pub description:Option<String>,
}