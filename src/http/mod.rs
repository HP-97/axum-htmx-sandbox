use std::sync::LazyLock;

use anyhow::Context;
use axum::Router;
use axum_embed::ServeEmbed;
use minijinja::Environment;
use rust_embed::Embed;
use tower_http::compression::CompressionLayer;

pub mod error;
pub mod forms;
pub mod healthcheck;
pub mod index;
pub mod spin;

#[derive(Embed, Clone)]
#[folder = "assets/"]
pub struct Asset;

static ENV: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();

    minijinja_embed::load_templates!(&mut env);
    // env.set_loader(path_loader("templates"));
    env
});

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

/// TODO: Ensure http port is configurable
pub async fn serve() -> anyhow::Result<()> {
    let ip = "0.0.0.0:8081";

    let app = full_router();

    let tcp_listener = tokio::net::TcpListener::bind(ip).await?;
    tracing::info!("serving HTTP server on {}", ip);
    axum::serve(tcp_listener, app)
        .await
        .context("error running HTTP server")
}

fn full_router() -> Router {
    let compression_layer = CompressionLayer::new().br(true).gzip(true);

    let app = Router::new()
        .merge(index::router())
        .merge(healthcheck::router())
        .merge(forms::router())
        .merge(using_serve_dir())
        .layer(compression_layer);

    app
}

/// Serve all of the static assets in /assets
fn using_serve_dir() -> Router {
    let serve_assets = ServeEmbed::<Asset>::new();
    Router::new().nest_service("/assets", serve_assets)
}
