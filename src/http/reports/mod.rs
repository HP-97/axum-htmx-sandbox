use axum::{
    http::{header, HeaderValue},
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
            "/reports",
            get(|| async { Redirect::permanent("/reports/") }),
        )
        .route("/reports/", get(reports_list))
        .layer(
            ServiceBuilder::new().layer(SetResponseHeaderLayer::overriding(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/html"),
            )),
        )
}

pub async fn reports_list() -> impl IntoResponse {
    let tmpl = ENV.get_template("report_list.html").unwrap();
    let ctx = context!();

    // (
    //     [("Hx-Trigger-After-Swap", "init")],
    //     tmpl.render(ctx).unwrap(),
    // )
    tmpl.render(ctx).unwrap()
}
