use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Book {
    pub id: Uuid,
    pub name: String,
    pub url: Option<Url>,
}

impl Book {
    #[allow(dead_code)]
    pub fn new(name: &str, url: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from(name),
            url: match url {
                Some(u) => Some(Url::from_str(u).unwrap()),
                None => None
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct BookCreateInput {
    pub name: String,
    pub url: Option<Url>,
}

impl BookCreateInput {
    pub fn to_model(&self) -> Book {
        Book {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            url: match &self.url {
                Some(u) => Some(u.clone()),
                None => None,
            },
        }
    }
}

pub type BookList = Vec<Book>;

#[async_trait]
pub trait BookService {
    async fn register(&self, book_create_input: &BookCreateInput) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_one(&self, id: &Uuid) -> Result<Book, Box<dyn std::error::Error>>;
    async fn get_all(&self) -> Result<BookList, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait BookRepo {
    async fn create(&self, book: &Book) -> Result<(), Box<dyn std::error::Error>>;
    async fn find_one(&self, id: &Uuid) -> Result<Book, Box<dyn std::error::Error>>;
    async fn find_all(&self) -> Result<BookList, Box<dyn std::error::Error>>;
}

