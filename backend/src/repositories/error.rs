use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Entity not found")]
    NotFound,
}

impl RepositoryError {
    /// Convert from sqlx::Error (useful for explicit conversions)
    pub fn from_sqlx(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}
