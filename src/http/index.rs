use axum::{
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};
use minijinja::context;
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;

use super::ENV;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .layer(
            ServiceBuilder::new().layer(SetResponseHeaderLayer::overriding(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html"),
            )),
        )
}

pub async fn index() -> impl IntoResponse {
    let tmpl = ENV.get_template("index.html").unwrap();
    let ctx = context!();
    (
        [("Hx-Trigger-After-Swap", "init")],
        tmpl.render(ctx).unwrap(),
    )
}
