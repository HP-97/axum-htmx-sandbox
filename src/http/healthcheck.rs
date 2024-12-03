//! Provides a health check on web application
//!

use axum::{
    http::{
        header,
        HeaderValue,
    },
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use minijinja::context;
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;

use super::ENV;

pub fn router() -> Router {
    Router::new()
        // All HTML routes go here!
        .route(
            "/healthcheck",
            get(|| async { Redirect::permanent("/healthcheck/") }),
        )
        .route("/healthcheck/", get(index))
        .layer(
            ServiceBuilder::new().layer(SetResponseHeaderLayer::overriding(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html"),
            )),
        )
}

pub async fn index() -> impl IntoResponse {
    let tmpl = ENV.get_template("healthcheck.html").unwrap();
    let ctx = context!();

    // ([("Hx-Trigger-After-Settle", "doHealthCheck")], tmpl.render(ctx).unwrap())
    tmpl.render(ctx).unwrap()
}
