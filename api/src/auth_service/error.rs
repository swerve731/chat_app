use super::claims::error::*;
use super::user::error::*;

use axum::response::{IntoResponse, Response};
use derive_more::From;

#[derive(Debug, From)]
pub enum AuthError {
    #[from]
    Signin(SignInError),
    #[from]
    Signup(SignUpError),
    #[from]
    Claims(ClaimsError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::Signin(e) => e.into_response(),
            Self::Signup(e) => e.into_response(),
            Self::Claims(e) => e.into_response(),
        }
    }
}
