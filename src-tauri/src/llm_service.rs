use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_base: String,
    pub model: String,
    #[serde(default = "default_batch_size")]
    pub batch_size: u32,
}

fn default_batch_size() -> u32 {
    5
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedBasicInfo {
    pub title: String,
    pub magnet_link: String,
    pub file_size: Option<String>,
    pub source_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchExtractBasicInfoResult {
    pub results: Vec<ExtractedBasicInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedAnalysisResult {
    pub title: String,
    pub purity_score: u8,
    pub tags: Vec<String>,
    pub magnet_link: String,
    pub file_size: Option<String>,
    pub file_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchAnalysisItem {
    pub title: String,
    pub file_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchAnalysisResult {
    pub cleaned_title: String,
    pub purity_score: u8,
    pub tags: Vec<String>,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn batch_extract_basic_info_from_html(
        &self,
        html_content: &str,
        extraction_config: &LlmConfig,
    ) -> Result<BatchExtractBasicInfoResult>;

    async fn batch_analyze_scores_and_tags(
        &self,
        original_title: &str,
        file_list: &[String],
        analysis_config: &LlmConfig,
    ) -> Result<(String, u8, Vec<String>)>;

    async fn batch_analyze_multiple_items(
        &self,
        items: &[BatchAnalysisItem],
        analysis_config: &LlmConfig,
    ) -> Result<Vec<BatchAnalysisResult>>;
}

pub struct OpenAiCompatibleClient {
    client: Client,
}

impl OpenAiCompatibleClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(90))
            .build()
            .expect("Failed to create LLM HTTP client");
        Self { client }
    }

    async fn complete_json<T: DeserializeOwned>(
        &self,
        config: &LlmConfig,
        system_prompt: &str,
        user_prompt: String,
    ) -> Result<T> {
        validate_config(config)?;

        let request = ChatCompletionRequest {
            model: config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user",
                    content: user_prompt,
                },
            ],
            temperature: 0.0,
            max_tokens: 8192,
            response_format: supports_json_mode(config).then_some(ResponseFormat {
                kind: "json_object",
            }),
        };

        let url = chat_completions_url(&config.api_base);
        for attempt in 0..3 {
            let response = self
                .client
                .post(&url)
                .bearer_auth(config.api_key.trim())
                .json(&request)
                .send()
                .await?;

            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            if status.is_success() {
                let response: ChatCompletionResponse = serde_json::from_str(&body)
                    .map_err(|e| anyhow!("Invalid chat completion response: {e}"))?;
                let content = response
                    .choices
                    .first()
                    .map(|choice| choice.message.content.as_str())
                    .filter(|content| !content.trim().is_empty())
                    .ok_or_else(|| anyhow!("Chat completion response did not contain content"))?;
                return parse_json_content(content);
            }

            if (status.as_u16() == 429 || status.is_server_error()) && attempt < 2 {
                tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                continue;
            }

            return Err(anyhow!(
                "{} request failed (HTTP {}): {}",
                provider_label(&config.provider),
                status,
                safe_error_body(&body)
            ));
        }

        Err(anyhow!("AI request failed after retries"))
    }
}

impl Default for OpenAiCompatibleClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LlmClient for OpenAiCompatibleClient {
    async fn batch_extract_basic_info_from_html(
        &self,
        html_content: &str,
        extraction_config: &LlmConfig,
    ) -> Result<BatchExtractBasicInfoResult> {
        let prompt = format!(
            r#"Extract every magnet-link result from the supplied HTML.

Return exactly one JSON object with a "results" array. Each item must contain:
- "title": nearby human-readable title with HTML removed
- "magnet_link": complete link beginning with magnet:?xt=urn:btih:
- "file_size": nearby size text or null
- "source_url": nearby detail-page URL or null

Do not invent, rewrite, or infer missing values. Preserve source order. If nothing is found,
return {{"results":[]}}.

HTML:
```html
{}
```"#,
            html_content
        );

        self.complete_json(
            extraction_config,
            "You are a deterministic HTML data extraction service. Output valid JSON only.",
            prompt,
        )
        .await
    }

    async fn batch_analyze_scores_and_tags(
        &self,
        original_title: &str,
        file_list: &[String],
        analysis_config: &LlmConfig,
    ) -> Result<(String, u8, Vec<String>)> {
        let results = self
            .batch_analyze_multiple_items(
                &[BatchAnalysisItem {
                    title: original_title.to_string(),
                    file_list: file_list.to_vec(),
                }],
                analysis_config,
            )
            .await?;

        let result = results
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Analysis returned no results"))?;
        Ok((result.cleaned_title, result.purity_score, result.tags))
    }

    async fn batch_analyze_multiple_items(
        &self,
        items: &[BatchAnalysisItem],
        analysis_config: &LlmConfig,
    ) -> Result<Vec<BatchAnalysisResult>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let input = serde_json::to_string(items)?;
        let prompt = format!(
            r#"Analyze each input item and return exactly one JSON object with a "results" array
in the same order and with the same number of items.

For each result:
- "cleaned_title": retain only work names and season/episode information
- "purity_score": integer 0-100 based only on the supplied real file_list; when file_list is
  empty, use 50 because purity is unknown
- "tags": at most four factual tags inferred from the title, ordered as quality, language,
  subtitle language, special format

Do not invent file names or facts. Output valid JSON only.

Input:
{}"#,
            input
        );

        #[derive(Deserialize)]
        struct BatchResponse {
            results: Vec<BatchAnalysisResult>,
        }

        let response: BatchResponse = self
            .complete_json(
                analysis_config,
                "You are a deterministic media metadata analysis service. Output valid JSON only.",
                prompt,
            )
            .await?;

        if response.results.len() != items.len() {
            return Err(anyhow!(
                "Analysis result count mismatch: expected {}, received {}",
                items.len(),
                response.results.len()
            ));
        }

        Ok(response.results)
    }
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    kind: &'static str,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

fn validate_config(config: &LlmConfig) -> Result<()> {
    match config.provider.as_str() {
        "agnes" | "deepseek" | "openai_compatible" => {}
        provider => return Err(anyhow!("Unsupported AI provider: {provider}")),
    }

    if config.api_key.trim().is_empty() {
        return Err(anyhow!("API key is required"));
    }
    if config.model.trim().is_empty() {
        return Err(anyhow!("Model is required"));
    }
    let url = url::Url::parse(config.api_base.trim())
        .map_err(|e| anyhow!("Invalid API base URL: {e}"))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(anyhow!("API base URL must use HTTP or HTTPS"));
    }
    Ok(())
}

fn supports_json_mode(config: &LlmConfig) -> bool {
    matches!(config.provider.as_str(), "deepseek" | "openai_compatible")
}

fn chat_completions_url(api_base: &str) -> String {
    let base = api_base.trim().trim_end_matches('/');
    if base.ends_with("/chat/completions") {
        base.to_string()
    } else {
        format!("{base}/chat/completions")
    }
}

fn parse_json_content<T: DeserializeOwned>(content: &str) -> Result<T> {
    let trimmed = content.trim();
    let without_fence = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```"))
        .unwrap_or(trimmed);
    let without_fence = without_fence.strip_suffix("```").unwrap_or(without_fence).trim();

    serde_json::from_str(without_fence).map_err(|e| {
        anyhow!(
            "Failed to parse structured AI response: {e}. Response prefix: {}",
            safe_error_body(without_fence)
        )
    })
}

fn safe_error_body(body: &str) -> String {
    body.chars().take(500).collect()
}

fn provider_label(provider: &str) -> &str {
    match provider {
        "agnes" => "Agnes",
        "deepseek" => "DeepSeek",
        "openai_compatible" => "OpenAI-compatible",
        _ => "AI provider",
    }
}

pub async fn test_connection(config: &LlmConfig) -> Result<String> {
    let client = OpenAiCompatibleClient::new();
    let response: serde_json::Value = client
        .complete_json(
            config,
            "Return valid JSON only.",
            r#"Return exactly {"status":"ok"}."#.to_string(),
        )
        .await?;

    if response.get("status").and_then(|value| value.as_str()) == Some("ok") {
        Ok(format!("{} connection successful", provider_label(&config.provider)))
    } else {
        Err(anyhow!("Provider responded, but structured output validation failed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[test]
    fn appends_chat_completions_path() {
        assert_eq!(
            chat_completions_url("https://apihub.agnes-ai.com/v1/"),
            "https://apihub.agnes-ai.com/v1/chat/completions"
        );
    }

    #[test]
    fn parses_fenced_json() {
        let value: serde_json::Value =
            parse_json_content("```json\n{\"status\":\"ok\"}\n```").unwrap();
        assert_eq!(value["status"], "ok");
    }

    #[test]
    fn rejects_removed_gemini_provider() {
        let config = LlmConfig {
            provider: "gemini".to_string(),
            api_key: "secret".to_string(),
            api_base: "https://example.com/v1".to_string(),
            model: "model".to_string(),
            batch_size: 5,
        };
        assert!(validate_config(&config).is_err());
    }

    #[tokio::test]
    async fn sends_openai_compatible_request_with_bearer_auth() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/chat/completions")
                .header("authorization", "Bearer secret");
            then.status(200)
                .header("content-type", "application/json")
                .json_body_obj(&serde_json::json!({
                    "choices": [{
                        "message": {
                            "content": "{\"status\":\"ok\"}"
                        }
                    }]
                }));
        });

        let config = LlmConfig {
            provider: "deepseek".to_string(),
            api_key: "secret".to_string(),
            api_base: server.base_url(),
            model: "deepseek-chat".to_string(),
            batch_size: 5,
        };

        let result = test_connection(&config).await.unwrap();
        mock.assert();
        assert!(result.contains("DeepSeek"));
    }
}
