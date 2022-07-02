use std::env;
use sqlx::postgres::PgPool;
use crate::PgPoolOptions;

pub async fn configure() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is required to setup database");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await.unwrap()
}