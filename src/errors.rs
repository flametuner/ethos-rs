use async_graphql::{Error, ErrorExtensions};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Connection Pool Error")]
    ConnectionPoolError,

    #[error("Load Error")]
    LoadError,
    #[error("Failed to create project")]
    FailedToCreate,
}

impl ErrorExtensions for StoreError {
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            StoreError::ConnectionPoolError => e.set("code", 500),
            StoreError::LoadError => {},
            StoreError::FailedToCreate => {}
        })
    }
}
