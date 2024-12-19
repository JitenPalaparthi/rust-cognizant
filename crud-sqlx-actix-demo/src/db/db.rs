use sqlx::{PgPool, Pool, Postgres};

pub async fn create_db_pool(dsn: &str) -> PgPool {
    PgPool::connect(dsn)
        .await
        .expect("failed to connect to database")
}
