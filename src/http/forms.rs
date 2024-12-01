use axum::{response::Html, routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/forms/basic", get(form_basic_get))
}

pub async fn form_basic_get() -> Html<&'static str> {
    Html("hello")

}

