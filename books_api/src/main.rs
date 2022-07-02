use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

mod infrastructure;
mod feature;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    if let Err(e) = dotenv::dotenv() {
        println!("Not applying .env : {:?}", e)
    }

    let pg_pool = Arc::new(infrastructure::postgres::configure().await);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(|cfg| configure_features(pg_pool.clone(), cfg))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn configure_features(pg_pool: Arc<PgPool>, cfg: &mut web::ServiceConfig) {
    configure_book(pg_pool.clone(), cfg);
}

fn configure_book(pg_pool: Arc<PgPool>, cfg: &mut web::ServiceConfig) {
    use crate::feature::book::book_service::BookServiceImpl;
    use crate::feature::book::book_repo::PostgresBookRepo;
    use crate::feature::book::rest_controller;

    let service = BookServiceImpl {
        book_repo: PostgresBookRepo {
            pg_pool: pg_pool.clone()
        }
    };
    rest_controller::configure(web::Data::new(service), cfg);
}