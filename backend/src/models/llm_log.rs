#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LlmLog {
    pub id: i32,
    pub model: String,
    pub message: String,
    pub timestamp: Option<String>,
    pub groupchat: String,
}

impl LlmLog {
    pub fn format(&self) -> String {
        format!("[{}]: {}", self.model, self.message)
    }
}

/// Input for creating a new LLM log entry (id and timestamp are auto-generated)
#[derive(Debug, Clone)]
pub struct CreateLlmLog {
    pub model: String,
    pub message: String,
    pub groupchat: String,
}
