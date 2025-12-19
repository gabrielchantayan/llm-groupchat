/**
 * Service for interacting with groupchats
 */
use crate::models::groupchat::Groupchat;
use crate::services::llm_service::LlmService;
use sqlx::SqlitePool;

pub struct GroupchatService {
    groupchat: Groupchat,
    llm_service: LlmService,
}

impl GroupchatService {
    pub fn new(groupchat: Groupchat, pool: SqlitePool) -> Self {
        let llm_service = LlmService::new(pool);
        Self {
            llm_service,
            groupchat,
        }
    }

    pub async fn chat(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Get random groupchat model
        let model = self.groupchat.models.random();

        // Send message to LLM
        let response = self.llm_service.chat(model).await?;

        Ok(response)
    }

    pub fn get_groupchat(&self) -> Groupchat {
        self.groupchat.clone()
    }

    pub fn get_participants(&self) -> Vec<String> {
        self.groupchat
            .models
            .models
            .iter()
            .map(|model| model.model_display_name.clone())
            .collect()
    }
}
