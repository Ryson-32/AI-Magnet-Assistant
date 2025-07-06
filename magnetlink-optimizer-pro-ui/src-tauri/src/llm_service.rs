// src-tauri/src/llm_service.rs

use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;
use anyhow::Result;

// --- 1. 定义与Gemini API交互所需的数据结构 ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

// 定义我们期望从Gemini收到的JSON对象的结构
#[derive(Serialize, Deserialize, Debug)]
pub struct AnalysisResult {
    pub purity_score: u8,
    pub content_type: String,
    pub video_quality: String,
    pub has_subtitles: bool,
    pub tags: Vec<String>,
}

// 批量分析结果
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchAnalysisResult {
    pub results: Vec<SingleAnalysisResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleAnalysisResult {
    pub title: String,
    pub purity_score: u8,
    pub content_type: String,
    pub video_quality: String,
    pub tags: Vec<String>,
}

// LLM客户端trait定义
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn evaluate_ad(&self, title: &str) -> Result<f32>;
    async fn enrich_result(&self, title: &str) -> Result<Vec<String>>;
    async fn batch_analyze(&self, titles: &[String]) -> Result<BatchAnalysisResult>;
}

// Gemini客户端实现
pub struct GeminiClient {
    config: LlmConfig,
    client: Client,
}

impl GeminiClient {
    pub fn new(config: LlmConfig) -> Self {
        let client = Client::new();
        Self { config, client }
    }
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn evaluate_ad(&self, title: &str) -> Result<f32> {
        // 单个广告评估的简化实现
        let analysis = self.analyze_single_resource(title.to_string(), vec![]).await
            .map_err(|e| anyhow::anyhow!("Failed to evaluate ad: {}", e))?;

        // 将purity_score转换为ad_score (分数越高表示越纯净，ad_score越低)
        let ad_score = (100 - analysis.purity_score) as f32 / 100.0;
        Ok(ad_score)
    }

    async fn enrich_result(&self, title: &str) -> Result<Vec<String>> {
        let analysis = self.analyze_single_resource(title.to_string(), vec![]).await
            .map_err(|e| anyhow::anyhow!("Failed to enrich result: {}", e))?;
        Ok(analysis.tags)
    }

    async fn batch_analyze(&self, titles: &[String]) -> Result<BatchAnalysisResult> {
        self.batch_analyze_resources(titles.to_vec()).await
            .map_err(|e| anyhow::anyhow!("Batch analysis failed: {}", e))
    }
}

// 定义发送给Gemini API的请求体结构
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// 定义从Gemini API收到的响应体结构 (我们只关心text部分)
#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize, Debug)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize, Debug)]
struct PartResponse {
    text: String,
}


// --- 2. 实现核心的API调用函数 ---

impl GeminiClient {
    // 单个资源分析（保持向后兼容）
    async fn analyze_single_resource(&self, title: String, file_list: Vec<String>) -> Result<AnalysisResult, String> {
        self.analyze_resource_with_gemini(title, file_list).await
    }

    // 批量资源分析 - 新的高效方法
    async fn batch_analyze_resources(&self, titles: Vec<String>) -> Result<BatchAnalysisResult, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.config.model, self.config.api_key
        );

        // 构建批量分析的prompt
        let titles_list = titles.iter()
            .enumerate()
            .map(|(i, title)| format!("{}. {}", i + 1, title))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"
            作为一名媒体文件分析专家，请分析以下磁力资源标题列表，为每个标题返回一个JSON对象。

            请返回一个包含"results"数组的JSON对象，数组中每个元素对应一个标题的分析结果，包含以下字段：
            - "title": 原始标题（字符串）
            - "purity_score": 0到100的整数，表示标题的纯净度（100=完全纯净，0=充满广告）
            - "content_type": 内容类型（"电影", "电视剧", "动漫", "纪录片", "综艺", "音乐", "体育", "软件", "游戏", "其他"）
            - "video_quality": 视频质量（"4K", "2160p", "1080p", "720p", "标清", "未知"）
            - "tags": 智能标签数组，如["科幻", "动作", "H.265"]

            **标题列表:**
            {}

            **输出要求:**
            请严格按照JSON格式返回，确保results数组中的元素顺序与输入标题列表一致。不要包含任何额外的解释或Markdown标记。
            "#,
            titles_list
        );

        // 构建请求体
        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let response = self.client.post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("网络请求失败: {}", e))?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(format!("API请求失败: {}", error_body));
        }

        let gemini_response = response.json::<GeminiResponse>().await
            .map_err(|e| format!("解析Gemini响应失败: {}", e))?;

        // 从响应中提取批量分析结果
        if let Some(candidate) = gemini_response.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                let cleaned_text = part.text.trim().replace("```json", "").replace("```", "");

                let result: BatchAnalysisResult = serde_json::from_str(&cleaned_text)
                    .map_err(|e| format!("解析批量分析JSON失败: {}. Raw text: {}", e, cleaned_text))?;

                return Ok(result);
            }
        }

        Err("Gemini响应中未找到有效内容".to_string())
    }

    // 原有的单个分析方法（重构为内部方法）
    async fn analyze_resource_with_gemini(&self, title: String, file_list: Vec<String>) -> Result<AnalysisResult, String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.config.model, self.config.api_key
        );

    let files_str = file_list.join("\n");

    // 构建我们在指南中定义的Prompt
    let prompt = format!(
        r#"
        作为一名媒体文件分析专家，请根据以下磁力资源的标题和文件列表，提取并返回一个JSON对象。
        这个JSON对象必须包含以下字段：
        - "purity_score": 一个0到100的整数，表示资源标题和文件列表中不含任何形式广告或无关推广信息（如网址、论坛名称、压制组名称等）的纯净度。100分表示完全纯净，0分表示充满广告。
        - "content_type": 一个字符串，表示内容的核心类型。可能的值包括："电影", "电视剧", "动漫", "纪录片", "综艺", "音乐", "体育", "软件", "游戏", "其他"。
        - "video_quality": 一个字符串，表示视频质量。如果无法判断则为"未知"。可能的值包括："4K", "2160p", "1080p", "720p", "标清", "未知"。
        - "has_subtitles": 一个布尔值，如果文件列表中包含字幕文件（如.srt, .ass, .sub），则为true，否则为false。
        - "tags": 一个字符串数组，包含从标题和文件名中提取的、描述内容属性的智能标签。例如：["科幻", "动作", "系列电影", "H.265", "DDP5.1"]。

        **输入信息:**
        - 标题: {}
        - 文件列表:
        {}

        **输出要求:**
        请严格按照JSON格式返回，不要包含任何额外的解释或Markdown标记。
        "#,
        title, files_str
    );

    // 构建请求体
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
    };

    let response = self.client.post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;
  
    if !response.status().is_success() {
        let error_body = response.text().await.unwrap_or_default();
        return Err(format!("API请求失败: {}", error_body));
    }

    let gemini_response = response.json::<GeminiResponse>().await
        .map_err(|e| format!("解析Gemini响应失败: {}", e))?;

    // 从复杂的响应结构中提取出核心的文本内容
    if let Some(candidate) = gemini_response.candidates.get(0) {
        if let Some(part) = candidate.content.parts.get(0) {
            let cleaned_text = part.text.trim().replace("```json", "").replace("```", "");
          
            // 将文本解析为我们定义的AnalysisResult结构体
            let result: AnalysisResult = serde_json::from_str(&cleaned_text)
                .map_err(|e| format!("解析最终JSON失败: {}. Raw text: {}", e, cleaned_text))?;
          
            return Ok(result);
        }
    }

        Err("Gemini响应中未找到有效内容".to_string())
    }
}

// 公共API函数（保持向后兼容）
pub async fn analyze_resource_with_gemini(title: String, file_list: Vec<String>, config: &LlmConfig) -> Result<AnalysisResult, String> {
    let client = GeminiClient::new(config.clone());
    client.analyze_resource_with_gemini(title, file_list).await
}

// 新的批量分析API
pub async fn batch_analyze_resources_with_gemini(titles: Vec<String>, config: &LlmConfig) -> Result<BatchAnalysisResult, String> {
    let client = GeminiClient::new(config.clone());
    client.batch_analyze_resources(titles).await
}

pub async fn test_connection(config: &LlmConfig) -> Result<String, String> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        config.model, config.api_key
    );

    let prompt = "你好，请确认你能收到这条消息。";

    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt.to_string() }],
        }],
    };

    let client = Client::new();
    let response = client.post(&url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if response.status().is_success() {
        Ok("连接成功".to_string())
    } else {
        let error_body = response.text().await.unwrap_or_default();
        Err(format!("API连接失败: {}", error_body))
    }
}