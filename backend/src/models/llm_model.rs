use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct LlmModel {
    pub model_name: String,
    pub openrouter_model_id: String,
    pub model_display_name: String,
    pub model_custom_instructions: Option<String>,
}

impl LlmModel {
    pub fn new(
        model_name: String,
        openrouter_model_id: String,
        model_display_name: String,
        model_custom_instructions: Option<String>,
    ) -> Self {
        Self {
            model_name,
            openrouter_model_id,
            model_display_name,
            model_custom_instructions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderedLlmModels {
    pub models: Vec<LlmModel>,
}

impl OrderedLlmModels {
    pub fn new(models: Vec<LlmModel>) -> Self {
        Self { models }
    }

    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &LlmModel)> {
        self.models.iter().enumerate()
    }

    pub fn random(&self) -> LlmModel {
        self.models
            .choose(&mut rand::thread_rng())
            .cloned()
            .expect("No models found")
    }
}
