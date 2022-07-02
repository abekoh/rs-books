use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

use crate::feature::book::ports::{Book, BookList, BookRepo};

pub struct PostgresBookRepo {
    pub pg_pool: Arc<PgPool>,
}

#[derive(sqlx::FromRow)]
pub struct BookDto {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub url: Option<String>,
    pub published_year: Option<i32>,
    pub original_published_year: Option<i32>,
}

impl BookDto {
    fn new(model: &Book) -> Result<Self, Box<dyn Error>> {
        let dto = BookDto {
            id: sqlx::types::Uuid::parse_str(&model.id.to_string())?,
            name: model.name.clone(),
            url: model.url.as_ref().map(|url| url.to_string()),
            published_year: None,
            original_published_year: None,
        };
        Ok(dto)
    }
    fn to_model(&self) -> Book {
        Book {
            id: *Uuid::from_bytes_ref(self.id.as_bytes()),
            name: self.name.clone(),
            url: match &self.url {
                Some(url) => {
                    match Url::from_str(url) {
                        Ok(u) => Some(u),
                        Err(_) => None,
                    }
                }
                None => None,
            },
        }
    }
}

impl PostgresBookRepo {
    async fn update_nullable(&self, dto: &BookDto) -> Result<(), Box<dyn Error>> {
        if dto.url.is_some() {
            let _ = sqlx::query!("UPDATE books SET url = $1 WHERE id = $2", &dto.url.as_ref().unwrap(), &dto.id)
                .execute(&*self.pg_pool)
                .await?;
        }
        if dto.published_year.is_some() {
            let _ = sqlx::query!("UPDATE books SET published_year = $1 WHERE id = $2", &dto.published_year.unwrap(), &dto.id)
                .execute(&*self.pg_pool)
                .await?;
        }
        if dto.original_published_year.is_some() {
            let _ = sqlx::query!("UPDATE books SET original_published_year = $1 WHERE id = $2", &dto.original_published_year.unwrap(), &dto.id)
                .execute(&*self.pg_pool)
                .await?;
        }
        Ok(())
    }
}

#[async_trait]
impl BookRepo for PostgresBookRepo {
    async fn create(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        let dto = BookDto::new(book)?;
        let _ = sqlx::query!("INSERT INTO books (id, name) VALUES ($1, $2)", &dto.id, &dto.name)
            .execute(&*self.pg_pool)
            .await?;
        self.update_nullable(&dto).await?;
        Ok(())
    }


    async fn find_one(&self, id: &Uuid) -> Result<Book, Box<dyn Error>> {
        let uid = sqlx::types::Uuid::from_bytes(*id.as_bytes());
        let res = sqlx::query_as!(BookDto, "SELECT id, name, url, published_year, original_published_year FROM books WHERE id = $1", &uid)
            .fetch_one(&*self.pg_pool)
            .await;
        match res {
            Ok(b) => Ok(b.to_model()),
            Err(e) => Err(Box::from(e))
        }
    }

    async fn find_all(&self) -> Result<BookList, Box<dyn Error>> {
        let res = sqlx::query_as!(BookDto, "SELECT id, name, url, published_year, original_published_year FROM books ORDER BY updated_at DESC LIMIT 10")
            .fetch_all(&*self.pg_pool)
            .await;
        match res {
            Ok(bs) => Ok(bs.iter().map(|book| book.to_model()).collect()),
            Err(e) => Err(Box::from(e))
        }
    }

    async fn update_one(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        let dto = BookDto::new(book)?;
        let _ = sqlx::query!("UPDATE books SET name = $1 WHERE id = $2", &dto.name, &dto.id)
            .execute(&*self.pg_pool)
            .await?;
        self.update_nullable(&dto).await?;
        Ok(())
    }
}