pub mod auth_service;
pub mod db_service;
pub mod server;

use derive_more::From;
use server::ServerError;
// server::run([auth_service, messaging_service])
pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug, From)]
pub enum AppError {
    // -- fs
    // LimitTooHigh {
    //     actual: usize,
    //     max: usize,
    // },

    // -- Externals
    #[from]
    Auth(auth_service::error::AuthError),

    #[from]
    Io(std::io::Error),

    #[from]
    Server(ServerError),
}

// Note: Implement Display as debug, for Web and app error, as anyway those errors will need to be streamed as JSON probably
//       to be rendered for the end user.
impl core::fmt::Display for AppError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for AppError {}
