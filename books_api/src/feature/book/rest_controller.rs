use actix_web::{HttpResponse, Responder, web};
use actix_web::web::{Json, Path};
use log::warn;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::feature::book::ports::{BookCreateInput, BookService, BookUpdateInput};

pub fn configure<T: 'static + BookService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/books", web::post().to(register::<T>));
    cfg.route("/books", web::get().to(get_all::<T>));
    cfg.route("/books/{id}", web::get().to(get_one::<T>));
    cfg.route("/books/{id}", web::put().to(update::<T>));
}

async fn register<T: BookService>(service: web::Data<T>, body: Json<BookCreateInput>) -> impl Responder {
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
struct OnePath {
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

async fn get_one<T: BookService>(service: web::Data<T>, path: Path<OnePath>) -> impl Responder {
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

async fn update<T: BookService>(service: web::Data<T>, path: Path<OnePath>, body: Json<BookUpdateInput>) -> impl Responder {
    let res = service.update(&path.id, &body).await;
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
