use serde::Deserialize;
use url::Url;
use uuid::Uuid;
use yew::prelude::*;
use reqwasm::http::Request;

#[derive(Clone, PartialEq, Deserialize)]
struct Book {
    pub id: Uuid,
    pub name: String,
    pub url: Option<Url>,
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
    // let books: Vec<Book> = vec![
    //     Book {
    //         id: Uuid::new_v4(),
    //         name: String::from("hoge"),
    //         url: None,
    //     },
    //     Book {
    //         id: Uuid::new_v4(),
    //         name: String::from("fuga"),
    //         url: None,
    //     },
    // ];
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with_deps(move |_| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_books: Vec<Book> = Request::get("http://localhost:8090/books")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                books.set(fetched_books);
            });
            || ()
        }, ());
    }
    html! {
        <>
            <h1>{ "Books" }</h1>
            <div>
                <BooksList books={(*books).clone()} />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
