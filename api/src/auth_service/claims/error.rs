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
