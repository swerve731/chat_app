use derive_more::From;

use crate::auth_service;
use axum::{
    extract::{MatchedPath, State},
    http::{self, header, Request},
    response::{IntoResponse, Response},
    routing::*,
    Form, Router,
};
use http::Method;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
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

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

impl AppState {
    fn new(pool: PgPool) -> Self {
        AppState { pool }
    }
}

pub async fn run_server() -> Result<(), crate::ServerError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .expose_headers([http::header::AUTHORIZATION]);

    let pool = crate::db_service::get_connection_pool().await?;
    let app_state = AppState::new(pool);

    let auth_routes = auth_routes(app_state.clone());
    let app = Router::new()
        .route(
            "/health",
            get(|| async { "All good! will run cargo test with this request later" }),
        )
        .nest("/auth", auth_routes)
        .with_state(app_state)
        .layer(cors)
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

pub fn auth_routes(state: AppState) -> axum::Router<AppState> {
    axum::Router::new()
        .route("/signup", post(signup_service))
        .route("/signin", post(signin_service))
        .with_state(state)
}

#[derive(Serialize, Deserialize)]
pub struct AuthForm {
    email: String,
    password: String,
}

pub async fn signin_service(
    State(state): State<AppState>,
    Form(form): Form<AuthForm>,
) -> impl IntoResponse {
    let pool = &state.pool;

    let signin_res = auth_service::user::User::signin(pool, &form.email, &form.password).await;

    match signin_res {
        Ok(jwt_token) => {
            let headers = [(header::AUTHORIZATION, jwt_token.as_str())];
            (
                http::status::StatusCode::OK,
                headers,
                "Account successfully authenticated",
            )
                .into_response()
        }
        Err(e) => e.into_response(),
    }
}
pub async fn signup_service(
    State(state): State<AppState>,
    Form(form): Form<AuthForm>,
) -> impl IntoResponse {
    let pool = &state.pool;
    println!("HERE");
    let signup_res = auth_service::user::User::signup(pool, &form.email, &form.password).await;

    match signup_res {
        Ok(jwt_token) => {
            let headers = [(header::AUTHORIZATION, jwt_token.as_str())];
            (
                http::status::StatusCode::OK,
                headers,
                "Account successfully created",
            )
                .into_response()
        }
        Err(e) => e.into_response(),
    }
}
