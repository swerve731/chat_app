use super::claims::error::*;
use super::user::error::*;

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
