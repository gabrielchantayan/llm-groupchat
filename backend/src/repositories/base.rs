use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use super::error::RepositoryError;

/// Base repository trait defining common CRUD operations.
///
/// Each repository implementation specifies its own Entity, CreateInput, and UpdateInput types.
#[async_trait]
pub trait Repository: Send + Sync {
    type Entity;
    type CreateInput;
    type UpdateInput;

    /// Find an entity by its ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Self::Entity>, RepositoryError>;

    /// Create a new entity.
    async fn create(&self, input: Self::CreateInput) -> Result<Self::Entity, RepositoryError>;

    /// Update an existing entity.
    async fn update(
        &self,
        id: Uuid,
        input: Self::UpdateInput,
    ) -> Result<Self::Entity, RepositoryError>;

    /// Delete an entity by its ID. Returns true if a row was deleted.
    async fn delete(&self, id: Uuid) -> Result<bool, RepositoryError>;
}

/// Shared context for repositories containing the database connection pool.
#[derive(Clone)]
pub struct RepositoryContext {
    pub pool: SqlitePool,
}

impl RepositoryContext {
    /// Create a new repository context with the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Begin a new database transaction.
    pub async fn transaction(
        &self,
    ) -> Result<sqlx::Transaction<'_, sqlx::Sqlite>, RepositoryError> {
        self.pool.begin().await.map_err(RepositoryError::from_sqlx)
    }
}
