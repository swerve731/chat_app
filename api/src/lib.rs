pub mod auth_service;

use derive_more::From;
// server::run([auth_service, messaging_service])
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- fs
    LimitTooHigh { actual: usize, max: usize },

    // -- Externals
    #[from]
    Io(std::io::Error),
}

// Note: Implement Display as debug, for Web and app error, as anyway those errors will need to be streamed as JSON probably 
//       to be rendered for the end user. 
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
