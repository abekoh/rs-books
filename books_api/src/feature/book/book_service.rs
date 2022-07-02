use std::error::Error;

use async_trait::async_trait;
use uuid::Uuid;

use crate::feature::book::ports::{Book, BookRepo, BookService};

pub struct BookServiceImpl<A: BookRepo> {
    pub book_repo: A,
}

#[async_trait]
impl<A> BookService for BookServiceImpl<A> where A: BookRepo + Sync + Send {
    async fn register(&self, book: &Book) -> Result<(), Box<dyn Error>> {
        self.book_repo.create(book).await
    }

    async fn get_one(&self, id: &Uuid) -> Result<Book, Box<dyn Error>> {
        self.book_repo.find_one(id).await
    }
}