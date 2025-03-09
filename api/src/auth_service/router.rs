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
        .route("/search", post(search_service))
        .with_state(state)
}

use super::user::User;

#[derive(Serialize, Deserialize)]
pub struct SearchForm {
    pub email: String,
}

pub async fn search_service(
    State(state): State<AppState>,
    Form(form): Form<SearchForm>,
) -> impl IntoResponse {
    let pool = &state.pool;

    let search_res = User::search_users(pool, &form.email).await;

    match search_res {
        Ok(users) => {
            if let Ok(user_json) = serde_json::to_string(&users) {
                (StatusCode::OK, user_json).into_response()
            } else {
                tracing::error!(
                    "An unexpected error occured searching users while converting users to json"
                );

                (StatusCode::INTERNAL_SERVER_ERROR, "An unkown error occured").into_response()
            }
        }
        Err(e) => e.into_response(),
    }
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

    let signin_res = User::signin(pool, &form.email, &form.password).await;

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
    let signup_res = User::signup(pool, &form.email, &form.password).await;

    match signup_res {
        Ok(jwt_token) => {
            let headers = [(header::AUTHORIZATION, jwt_token.as_str())];
            (StatusCode::OK, headers, "Account successfully created").into_response()
        }
        Err(e) => e.into_response(),
    }
}
