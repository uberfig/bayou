// https://github.com/astro/sigh/blob/main/src/error.rs

use std::string::FromUtf8Error;

/// General error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Cryptographic issue
    #[error("Cryptographic issue")]
    OpenSsl(#[from] openssl::error::ErrorStack),
    /// Invalid UTF-8
    #[error("Invalid UTF-8")]
    Utf8(#[from] FromUtf8Error),
}
