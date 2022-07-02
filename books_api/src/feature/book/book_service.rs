use std::error::Error;

use async_trait::async_trait;
use uuid::Uuid;

use crate::feature::book::ports::{Book, BookCreateInput, BookList, BookRepo, BookService, BookUpdateInput};

pub struct BookServiceImpl<A: BookRepo> {
    pub book_repo: A,
}

#[async_trait]
impl<A> BookService for BookServiceImpl<A> where A: BookRepo + Sync + Send {
    async fn register(&self, input: &BookCreateInput) -> Result<(), Box<dyn Error>> {
        self.book_repo.create(&input.to_model()).await
    }

    async fn get_one(&self, id: &Uuid) -> Result<Book, Box<dyn Error>> {
        self.book_repo.find_one(id).await
    }

    async fn get_all(&self) -> Result<BookList, Box<dyn Error>> {
        self.book_repo.find_all().await
    }

    async fn update(&self, id: &Uuid, input: &BookUpdateInput) -> Result<(), Box<dyn Error>> {
        let prev = self.book_repo.find_one(id).await?;
        let updated = input.to_model(&prev);
        self.book_repo.update(&updated).await
    }

    async fn delete(&self, id: &Uuid) -> Result<(), Box<dyn Error>> {
        self.book_repo.delete(id).await
    }
}