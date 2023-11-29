use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "index.html")]

pub struct MyTemplate {}

pub async fn index() -> Html<String> {
    Html(MyTemplate {}.render().unwrap())
}

pub async fn hello_world() -> Html<&'static str> {
    Html("Hello Nigga!")
}
