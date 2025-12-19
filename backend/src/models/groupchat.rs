use crate::models::llm_model::OrderedLlmModels;

#[derive(Debug, Clone)]
pub struct Groupchat {
    pub models: OrderedLlmModels,
}
