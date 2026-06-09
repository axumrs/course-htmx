use axum::{Router, routing::get};
use tokio::net::TcpListener;

mod asset;
mod handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9527").await?;
    let app = Router::new()
        .route("/", get(handler::web::index))
        .route("/profile", get(handler::web::profile_api))
        .fallback(handler::static_file::static_handler);
    axum::serve(listener, app).await?;
    Ok(())
}
