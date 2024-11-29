use axum::Router;

/// TODO: Ensure http port is configurable 
pub async fn serve() -> anyhow::Result<()> {
    let ip = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(ip).await?;
    // axum::serve(tcp_listener, make_service)
    unimplemented!();
}

fn full_router() -> Router {
    unimplemented!();
}
