use reqwasm::http::Request;
use serde::Deserialize;
use url::Url;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

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
    let book_lines: Html = books.iter().map(|book| {
        html! {
            <tr key={ book.id.to_string() }>
                <td>{ book.id }</td>
                <td>{ book.name.clone() }</td>
                // <td><a href={ book.url.clone().unwrap().to_string() }>{ for book.url.clone() }</a></td>
                <td>if book.url.is_some() {
                    <a href={ book.url.clone().unwrap().to_string() }>{ for book.url.clone() }</a>
                }</td>
            </tr>
        }
    }).collect();
    html! {
        <table>
            <thead>
                <tr>
                    <th>{ "ID" }</th>
                    <th>{ "Name" }</th>
                    <th>{ "URL" }</th>
                </tr>
            </thead>
            <tbody>
                {book_lines}
            </tbody>
        </table>
    }
}

#[function_component(Home)]
fn home() -> Html {
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

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
