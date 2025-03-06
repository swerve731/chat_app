use axum::{routing::*, Router};
use derive_more::From;

#[derive(Debug, From)]
pub enum ServerError {
    Axum(axum::Error),
    TokioIo(tokio::io::Error),
    Database(sqlx::Error),
}

const HOST_PORT: &str = "0.0.0.0:3000";
pub async fn run_server() -> Result<(), crate::ServerError> {
    let pool = crate::db_service::get_connection_pool().await?;

    let app = Router::new().route(
        "/health",
        get(|| async { "All good! will run cargo test with this request later" }),
    );

    let listener = tokio::net::TcpListener::bind(HOST_PORT).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
