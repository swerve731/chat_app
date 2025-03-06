use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use derive_more::From;

#[derive(Debug, From)]
pub enum ClaimsError {
    #[from]
    Jwt(jsonwebtoken::errors::Error),

    TokenExpired {
        exp: usize,
        current_time: usize,
    },
}

impl IntoResponse for ClaimsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Jwt(e) => {
                tracing::error!("Jwt error: {} ", e.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unkown error occured.",
                )
                    .into_response()
            }
            Self::TokenExpired { exp, current_time } => {
                tracing::debug!("Jwt token expired");
                // this will redirect the user to the home page
                Redirect::to("/").into_response()
            }
        }
    }
}
