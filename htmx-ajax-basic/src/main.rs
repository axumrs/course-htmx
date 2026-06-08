use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
mod api;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9527").await?;
    let app = Router::new()
        .nest("/api", api_router())
        .fallback_service(ServeDir::new("./html_files"));

    axum::serve(listener, app).await?;
    Ok(())
}

fn api_router() -> Router {
    Router::new().route("/messages", get(api::message_list))
}
