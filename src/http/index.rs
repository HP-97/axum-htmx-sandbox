use axum::{response::Html, routing::get, Router};
use minijinja::context;

use super::ENV;

pub fn router() -> Router {
    Router::new().route("/", get(index))
}

pub async fn index() -> Html<String> {
    let tmpl = ENV.get_template("index.html").unwrap();
    let ctx = context!();
    Html(tmpl.render(ctx).unwrap())
}
