use derive_more::From;
use std::str::FromStr;

pub type AppResult<T> = core::result::Result<T, AppError>;

#[derive(Debug, Clone, From)]
pub enum AppError {
    NotFound,

    // ---
    ServerError { code: i64 },
    TryLater,
    Unauthorized,
    CannotSetCookie,
    CannotConvertToString,

    // -- Server
    ServerFunctionError(String),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for AppError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for AppError {}

// endregion: --- Error Boilerplate

impl FromStr for AppError {
    type Err = Self;

    fn from_str(_s: &str) -> AppResult<Self> {
        Ok(Self::Unauthorized)
    }
}
