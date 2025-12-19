// Service for interacting with LLMs
use crate::models::llm_log::CreateLlmLog;
use crate::models::llm_model::LlmModel;
use crate::repositories::llm_log_repo::LlmLogRepo;
use openrouter_rs::{
    OpenRouterClient, api::chat::*, types::Role, types::completion::CompletionsResponse,
};
use sqlx::SqlitePool;
use std::env;

pub struct LlmService {
    client: OpenRouterClient,
    message_history_length: u8,
    llm_log_repo: LlmLogRepo,
}

impl LlmService {
    /// Creates a new LlmService, reading OPENROUTER_API_KEY from environment
    pub fn new(pool: SqlitePool) -> Self {
        let api_key = env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY must be set");

        let client = OpenRouterClient::builder().api_key(api_key).build();

        if client.is_err() {
            panic!("Failed to create OpenRouter client");
        }

        let client = client.unwrap();

        let message_history_length: u8 = env::var("MESSAGE_HISTORY_LENGTH")
            .unwrap_or("25".to_string())
            .parse()
            .unwrap();

        let llm_log_repo = LlmLogRepo::new(pool);

        Self {
            client,
            message_history_length,
            llm_log_repo,
        }
    }

    /// Send a chat message to the LLM and get a response
    pub async fn chat(&self, model: LlmModel) -> Result<String, Box<dyn std::error::Error>> {
        println!("Sending message to {}", model.model_name);

        // Get latest messages from database
        let logs = self
            .llm_log_repo
            .get_recent_logs(self.message_history_length)
            .await?;

        // Format the message history into a prompt
        let history = logs
            .iter()
            .map(|log| log.format())
            .collect::<Vec<String>>()
            .join("\n");

        let prompt = format!(
            "You are in a group chat with a bunch of different participants. Here is a list of previous messages:\n{}\n\nContinue on the conversation.\nIMPORTANT: Only include your message in the response.",
            history
        );

        // Build the request
        let request = ChatCompletionRequest::builder()
            .model(model.openrouter_model_id)
            .messages(vec![Message::new(Role::User, &prompt)])
            .build()?;

        // Call OpenRouter API
        let response: CompletionsResponse = self.client.send_chat_completion(&request).await?;

        // Extract message
        let message = response.choices[0].content();

        // message is Option<&str>
        // unwrap it
        let message = message.unwrap().trim().to_string();

        // Save the message to the database
        self.llm_log_repo
            .insert_log(CreateLlmLog {
                model: model.model_name.clone(),
                message: message.clone(),
                groupchat: "TODO".to_string(),
            })
            .await
            .unwrap();

        println!("Message from {}: {}", model.model_name, message);

        Ok(message)
    }
}
