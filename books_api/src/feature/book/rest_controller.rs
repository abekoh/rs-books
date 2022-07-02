use actix_web::{HttpResponse, Responder, web};
use actix_web::web::{Json, Path};
use log::warn;
use serde::Deserialize;
use uuid::Uuid;

use crate::feature::book::ports::{Book, BookService};

pub fn configure<T: 'static + BookService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/books", web::post().to(register::<T>));
    cfg.route("/books", web::get().to(get_all::<T>));
    cfg.route("/books/{id}", web::get().to(get_one::<T>));
}

async fn register<T: BookService>(service: web::Data<T>, body: Json<Book>) -> impl Responder {
    let res = service.register(&body).await;
    match res {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => match err {
            e => {
                warn!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[derive(Deserialize)]
struct GetOnePath {
    id: Uuid,
}

async fn get_all<T: BookService>(service: web::Data<T>) -> impl Responder {
    let res = service.get_all().await;
    match res {
        Ok(bs) => HttpResponse::Ok().json(bs),
        Err(err) => match err {
            e => {
                warn!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

async fn get_one<T: BookService>(service: web::Data<T>, path: Path<GetOnePath>) -> impl Responder {
    let res = service.get_one(&path.id).await;
    match res {
        Ok(b) => HttpResponse::Ok().json(b),
        Err(err) => match err {
            e => {
                warn!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
