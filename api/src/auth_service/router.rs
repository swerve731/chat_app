use crate::server::AppState;
use axum::{
    extract::{Form, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};

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

    let signin_res = super::user::User::signin(pool, &form.email, &form.password).await;

    match signin_res {
        Ok(jwt_token) => {
            let headers = [(header::AUTHORIZATION, jwt_token.as_str())];
            (
                StatusCode::OK,
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
    let signup_res = super::user::User::signup(pool, &form.email, &form.password).await;

    match signup_res {
        Ok(jwt_token) => {
            let headers = [(header::AUTHORIZATION, jwt_token.as_str())];
            (StatusCode::OK, headers, "Account successfully created").into_response()
        }
        Err(e) => e.into_response(),
    }
}
