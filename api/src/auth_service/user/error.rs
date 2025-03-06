use super::super::claims::error::ClaimsError;
use axum::{
    http::{self},
    response::IntoResponse,
};
use derive_more::From;

#[derive(Debug, From)]
pub enum SignInError {
    WrongPassword,
    UsernameNotFound {
        requested_username: String,
    },
    #[from]
    Database(sqlx::Error),

    #[from]
    JwtClaims(ClaimsError),
}

impl IntoResponse for SignInError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(e) => {
                tracing::error!("Database error while signingin user {:?}", e);
                http::status::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::UsernameNotFound { requested_username } => (
                http::status::StatusCode::NOT_FOUND,
                format!("Username: {}, Not found.", requested_username),
            )
                .into_response(),
            Self::WrongPassword => (
                http::StatusCode::UNAUTHORIZED,
                format!("Password incorrect, double check your credentials and try again."),
            )
                .into_response(),
            Self::JwtClaims(e) => e.into_response(),
        }
    }
}

#[derive(Debug, From)]
pub enum SignUpError {
    UsernameTaken {
        requested_username: String,
    },
    PasswordTooShort {
        min_length: usize,
        actual_length: usize,
    },

    PasswordTooWeak {
        has_lowercase: bool,
        has_uppercase: bool,
        has_number: bool,
        has_special: bool,
    },
    #[from]
    Database(sqlx::Error),
    #[from]
    JwtClaims(ClaimsError),
}

impl IntoResponse for SignUpError {
    fn into_response(self) -> axum::response::Response {
        match self {

                SignUpError::UsernameTaken { requested_username } => {
                    (
                        http::status::StatusCode::CONFLICT,
                        format!(
                            "Username {} is already taken. Please choose a different username.",
                            requested_username
                        ),
                    )
                        .into_response()
                }
                SignUpError::PasswordTooShort {
                    min_length,
                    actual_length,
                } => (
                    http::status::StatusCode::BAD_REQUEST,
                    format!(
                        "Password must be at least {} characters long. You provided a password of {} characters.",
                        min_length, actual_length
                    ),
                )
                    .into_response(),
                SignUpError::PasswordTooWeak {
                    has_lowercase,
                    has_uppercase,
                    has_number,
                    has_special,
                } => {
                    let mut password_requirements = std::collections::HashMap::new();
                        password_requirements.insert("lowercase", has_lowercase);
                        password_requirements.insert("uppercase", has_uppercase);
                        password_requirements.insert("number", has_number);
                        password_requirements.insert("special", has_special);

                    (
                        http::status::StatusCode::BAD_REQUEST,
                        axum::Json(password_requirements),
                    )
                        .into_response()
                }
                SignUpError::JwtClaims(e) =>{
                    e.into_response()
                    // (
                    //     http::status::StatusCode::INTERNAL_SERVER_ERROR,
                    //     "Error generating authentication token. Please try again later.".to_string(),
                    // )
                }
                    .into_response(),
                SignUpError::Database(e) => {
                    tracing::error!("Database error while signingup user {:?}", e);
                    http::status::StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
        }
    }
}
