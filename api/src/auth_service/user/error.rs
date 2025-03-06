use super::super::claims::error::ClaimsError;
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
}
