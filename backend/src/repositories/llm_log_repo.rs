// Repository for interacting with LLM logs

use sqlx::SqlitePool;

use super::error::RepositoryError;
use crate::models::llm_log::{CreateLlmLog, LlmLog};

/// Repository for LLM log operations
pub struct LlmLogRepo {
    pool: SqlitePool,
}

impl LlmLogRepo {
    /// Create a new LlmLogRepo with the given connection pool
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Insert a new log entry into the database.
    /// Returns the created log with its auto-generated id and timestamp.
    pub async fn insert_log(&self, input: CreateLlmLog) -> Result<LlmLog, RepositoryError> {
        let result = sqlx::query_as::<_, LlmLog>(
            r#"
            INSERT INTO llm_log (model, message, groupchat)
            VALUES (?, ?, ?)
            RETURNING id, model, message, timestamp, groupchat
            "#,
        )
        .bind(&input.model)
        .bind(&input.message)
        .bind(&input.groupchat)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    /// Retrieve the n most recent logs, sorted by most recent first.
    pub async fn get_recent_logs(&self, n: u8) -> Result<Vec<LlmLog>, RepositoryError> {
        let result = sqlx::query_as::<_, LlmLog>(
            r#"
            SELECT id, model, message, timestamp, groupchat
            FROM llm_log
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(n)
        .fetch_all(&self.pool)
        .await?;

        Ok(result as Vec<LlmLog>)
    }
}
