use derive_more::From;

use axum::{extract::MatchedPath, http::Request, response::Response, routing::*, Router};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, From)]
pub enum ServerError {
    Axum(axum::Error),
    TokioIo(tokio::io::Error),
    Database(sqlx::Error),
}

const HOST_PORT: &str = "0.0.0.0:3000";
pub async fn run_server() -> Result<(), crate::ServerError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = crate::db_service::get_connection_pool().await?;

    let app = Router::new()
        .route(
            "/health",
            get(|| async { "All good! will run cargo test with this request later" }),
        )
        .with_state(pool)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str)
                        .unwrap_or(request.uri().path());

                    info_span!(
                        "http_request",
                        method = %request.method(),
                        path = %path,
                    )
                })
                .on_request(|request: &Request<_>, _span: &Span| {
                    info!("{} {}", request.method(), request.uri().path());
                })
                .on_response(
                    |response: &Response, latency: std::time::Duration, _span: &Span| {
                        info!("status={} latency={:?}", response.status(), latency);
                    },
                ),
        );

    let listener = tokio::net::TcpListener::bind(HOST_PORT).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}
