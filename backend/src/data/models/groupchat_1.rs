use std::sync::LazyLock;

use crate::models::groupchat::Groupchat;
use crate::models::llm_model::{LlmModel, OrderedLlmModels};

pub static CLAUDE_HAIKU: LazyLock<LlmModel> = LazyLock::new(|| {
    LlmModel::new(
        "claude_haiku_4_5".to_string(),
        "anthropic/claude-haiku-4.5".to_string(),
        "Claude Haiku".to_string(),
        None,
    )
});

pub static OAI_GPT4O_MINI: LazyLock<LlmModel> = LazyLock::new(|| {
    LlmModel::new(
        "oai_gpt_4o_mini".to_string(),
        "openai/gpt-4o-mini".to_string(),
        "GPT-4o Mini".to_string(),
        None,
    )
});

pub static GOOGLE_GEMINI3_FLASH: LazyLock<LlmModel> = LazyLock::new(|| {
    LlmModel::new(
        "google_gemini_3_flash".to_string(),
        "google/gemini-3-flash-preview".to_string(),
        "Gemini Flash".to_string(),
        None,
    )
});

pub static MOONSHOT_KIMI_K2: LazyLock<LlmModel> = LazyLock::new(|| {
    LlmModel::new(
        "moonshot_kimi_k2".to_string(),
        "moonshotai/kimi-k2-0905".to_string(),
        "Kimi K2".to_string(),
        None,
    )
});

pub static XIAOMI_MIMO_V2_FLASH: LazyLock<LlmModel> = LazyLock::new(|| {
    LlmModel::new(
        "xiaomi_mimo_v2_flash".to_string(),
        "xiaomi/mimo-v2-flash:free".to_string(),
        "MiMo".to_string(),
        None,
    )
});

pub static MISTRAL_DEVSTRAL2: LazyLock<LlmModel> = LazyLock::new(|| {
    LlmModel::new(
        "mistral_devstral_2".to_string(),
        "mistralai/devstral-2512:free".to_string(),
        "Devstral".to_string(),
        None,
    )
});

pub static GROUPCHAT_1: LazyLock<Groupchat> = LazyLock::new(|| {
    Groupchat {
        models: OrderedLlmModels::new(vec![
            CLAUDE_HAIKU.clone(),
            OAI_GPT4O_MINI.clone(),
            GOOGLE_GEMINI3_FLASH.clone(),
            MOONSHOT_KIMI_K2.clone(),
            XIAOMI_MIMO_V2_FLASH.clone(),
            MISTRAL_DEVSTRAL2.clone(),
        ]),
    }
});
