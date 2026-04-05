use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Message in a conversation with the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Response from Ollama's native /api/chat endpoint
#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: Option<Message>,
}

/// Model info from Ollama's /api/tags
#[derive(Debug, Deserialize)]
struct OllamaTagsResponse {
    models: Option<Vec<OllamaModel>>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

/// Client for talking to Ollama (native API) with OpenAI-compat fallback
pub struct LlmClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
    is_ollama: bool,
}

/// The default model Clawling tries to use
pub const DEFAULT_MODEL: &str = "deepseek-r1:8b";

impl LlmClient {
    pub fn new(base_url: &str, model: Option<String>) -> Self {
        LlmClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            client: reqwest::Client::new(),
            is_ollama: false,
        }
    }

    /// Detect Ollama and check what models are available
    pub async fn detect(&mut self) -> DetectResult {
        // Try Ollama native API first
        let tags_url = format!("{}/api/tags", self.base_url);
        match self.client
            .get(&tags_url)
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
        {
            Ok(resp) => {
                if let Ok(tags) = resp.json::<OllamaTagsResponse>().await {
                    self.is_ollama = true;
                    let models = tags.models.unwrap_or_default();
                    let model_names: Vec<String> = models.iter().map(|m| m.name.clone()).collect();

                    if model_names.iter().any(|n| n.starts_with(&self.model.split(':').next().unwrap_or(&self.model).to_string())) {
                        return DetectResult::Ready { models: model_names };
                    } else if model_names.is_empty() {
                        return DetectResult::OllamaNoModels;
                    } else {
                        // Ollama is running but doesn't have our preferred model
                        // Use the first available model
                        self.model = model_names[0].clone();
                        return DetectResult::Ready { models: model_names };
                    }
                }
                // Server responded but not Ollama format — try OpenAI compat
                DetectResult::GenericServer
            }
            Err(_) => {
                // Try OpenAI-compatible endpoint as fallback
                let models_url = format!("{}/v1/models", self.base_url);
                if self.client
                    .get(&models_url)
                    .timeout(std::time::Duration::from_secs(3))
                    .send()
                    .await
                    .is_ok()
                {
                    DetectResult::GenericServer
                } else {
                    DetectResult::NoServer
                }
            }
        }
    }

    /// Send a conversation and get a response
    pub async fn chat(&self, messages: &[Message]) -> Result<String> {
        if self.is_ollama {
            self.chat_ollama(messages).await
        } else {
            self.chat_openai_compat(messages).await
        }
    }

    /// Chat via Ollama's native API
    async fn chat_ollama(&self, messages: &[Message]) -> Result<String> {
        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": false,
        });

        let response = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to reach Ollama")?;

        let chat_response: OllamaChatResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        chat_response
            .message
            .map(|m| m.content)
            .context("Ollama returned no response")
    }

    /// Chat via OpenAI-compatible API (LM Studio, llama.cpp server, etc.)
    async fn chat_openai_compat(&self, messages: &[Message]) -> Result<String> {
        #[derive(Deserialize)]
        struct ChatResponse { choices: Vec<Choice> }
        #[derive(Deserialize)]
        struct Choice { message: Message }

        let body = serde_json::json!({
            "messages": messages,
            "model": self.model,
            "temperature": 0.7,
            "max_tokens": 2048,
        });

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to reach LLM server")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse LLM response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .context("LLM returned no response")
    }

    pub fn model_name(&self) -> &str {
        &self.model
    }
}

/// Result of trying to detect and connect to an LLM server
pub enum DetectResult {
    /// Ollama found with models ready
    Ready { models: Vec<String> },
    /// Ollama running but no models pulled
    OllamaNoModels,
    /// Some non-Ollama server responding (LM Studio, llama.cpp, etc.)
    GenericServer,
    /// No server found at all
    NoServer,
}
