//! Provides a health check on web application
//!

use axum::{response::Html, routing::get, Router};
use minijinja::context;

use super::ENV;

pub fn router() -> Router {
    Router::new()
        .route("/healthcheck", get(index))
}

pub async fn index() -> Html<String> {
    let tmpl = ENV.get_template("healthcheck.html").unwrap();
    let ctx = context!();
    Html(tmpl.render(ctx).unwrap())
}

