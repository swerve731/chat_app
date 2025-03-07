use super::super::claims::error::ClaimsError;
use argon2::password_hash;
use axum::{
    http::{self},
    response::IntoResponse,
};
use derive_more::From;

#[derive(Debug, From)]
pub enum SignInError {
    WrongPassword,
    EmailNotFound {
        requested_email: String,
    },
    #[from]
    Database(sqlx::Error),

    #[from]
    JwtClaims(ClaimsError),

    #[from]
    PasswordHashing(argon2::password_hash::Error),
}

impl IntoResponse for SignInError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Database(e) => {
                tracing::error!("Database error while signingin user {:?}", e);
                http::status::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::EmailNotFound { requested_email } => (
                http::status::StatusCode::NOT_FOUND,
                format!("Email: {}, Not found.", requested_email),
            )
                .into_response(),
            Self::WrongPassword => (
                http::StatusCode::UNAUTHORIZED,
                format!("Password incorrect, double check your credentials and try again."),
            )
                .into_response(),
            Self::JwtClaims(e) => e.into_response(),
            Self::PasswordHashing(e) => {
                tracing::error!("Argon2 hashing error on signin {:?}", e);
                http::status::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[derive(Debug, From)]
pub enum SignUpError {
    InvalidEmail {
        requested_email: String,
    },
    EmailTaken {
        requested_email: String,
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

    #[from]
    PasswordHashing(password_hash::Error),
}

impl IntoResponse for SignUpError {
    fn into_response(self) -> axum::response::Response {
        match self {
                Self::InvalidEmail { requested_email } => {
                    (
                        http::status::StatusCode::BAD_REQUEST,
                        format!(
                            "Email {} is not a valid email. Please check your input and try again.",
                            requested_email
                        ),
                    )
                        .into_response()
                },
                SignUpError::EmailTaken { requested_email } => {
                    (
                        http::status::StatusCode::CONFLICT,
                        format!(
                            "Email {} is already taken. Please choose a different email, or try logging in.",
                            requested_email
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
                },
                Self::PasswordHashing(e) => {
                    tracing::error!("Argon2 hashing error in signup {:?}", e);
                    http::status::StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
        }
    }
}
