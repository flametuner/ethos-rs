use async_graphql::{Error, ErrorExtensions};
use ethers::types::SignatureError;
use fixed_hash::rustc_hex::FromHexError;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Connection Pool Error")]
    ConnectionPoolError(#[from] r2d2::Error),

    #[error("Not Found")]
    WalletNotFound(String),
    // #[error("Load Error")]
    // LoadError,
    #[error("Failed to create JWT")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    // #[error("Failed to create project")]
    // FailedToCreate,
    #[error("Address not valid")]
    InvalidAddress(#[from] FromHexError),

    #[error("Signature not valid")]
    InvalidSignature(#[from] SignatureError),

    #[error("There was a database Error")]
    DatabaseError(#[from] diesel::result::Error),
}

impl ErrorExtensions for StoreError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            StoreError::ConnectionPoolError(_) => e.set("code", 500),
            _ => {}
        })
    }
}
