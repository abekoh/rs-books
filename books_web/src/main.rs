use serde::Deserialize;
// use url::Url;
// use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Book {
    // pub id: Uuid,
    pub name: String,
    // pub url: Option<Url>,
}

#[derive(Properties, PartialEq)]
struct BookListProps {
    books: Vec<Book>,
}

#[function_component(BooksList)]
fn books_list(BookListProps { books }: &BookListProps) -> Html {
    books.iter().map(|book| {
        html! {
            <p>{book.name.clone()}</p>
        }
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let books: Vec<Book> = vec![
        Book {
            name: String::from("hoge"),
        },
        Book {
            name: String::from("fuga"),
        },
    ];
    html! {
        <>
            <h1>{ "Books" }</h1>
            <div>
                <BooksList books={books.clone()} />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
