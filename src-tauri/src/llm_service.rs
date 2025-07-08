// src-tauri/src/llm_service.rs

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// 智能处理API Base URL，为不同的API服务添加正确的路径
fn normalize_api_base(api_base: &str) -> String {
    // 移除末尾的斜杠，避免双斜杠问题
    let trimmed_base = api_base.trim_end_matches('/');

    // 如果是官方Gemini域名且没有包含/v1beta，则自动添加
    if trimmed_base == "https://generativelanguage.googleapis.com" {
        format!("{}/v1beta", trimmed_base)
    } else if trimmed_base.starts_with("https://generativelanguage.googleapis.com") && !trimmed_base.contains("/v1beta") {
        format!("{}/v1beta", trimmed_base)
    } else if (trimmed_base.starts_with("http://") || trimmed_base.starts_with("https://"))
        && !trimmed_base.contains("/v1beta")
        && !trimmed_base.contains("/api/")
        && !trimmed_base.contains("/v1/") {
        // 对于自定义代理服务器，如果没有包含API路径，尝试添加/v1beta
        // 这适用于Gemini Balance等代理服务
        format!("{}/v1beta", trimmed_base)
    } else {
        // 对于其他URL（包括已经包含路径的自定义代理），保持原样但移除末尾斜杠
        trimmed_base.to_string()
    }
}

// --- 0. 公共配置 ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

// --- 1. 第一阶段：从HTML中提取基础信息 ---

/// 第一阶段：从HTML中提取的单个原始、未经处理的磁力链接信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedBasicInfo {
    pub title: String,
    pub magnet_link: String,
    pub file_size: Option<String>,
}

/// 第一阶段：批量提取结果
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BatchExtractBasicInfoResult {
    pub results: Vec<ExtractedBasicInfo>,
}

// --- 2. 第二阶段：分析分数和标签 ---

/// 第二阶段：对单个磁力链接的文件列表进行详细分析后的最终结果
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedAnalysisResult {
    pub title: String,           // 精简后的标题
    pub purity_score: u8,        // 纯净度分数 (由LLM计算)
    pub tags: Vec<String>,       // 智能标签
    pub magnet_link: String,     // 原始磁力链接 (从第一阶段透传)
    pub file_size: Option<String>, // 原始文件大小 (从第一阶段透传)
    pub file_list: Vec<String>, // 文件列表
}

/// LLM为第二阶段分析返回的原始数据结构
#[derive(Serialize, Deserialize, Debug)]
struct LlmFileAnalysis {
    pub original_filename: String, // 原始文件名，用于匹配
    pub cleaned_title: String,     // 清理后的标题 (仅对主媒体文件有意义)
    pub tags: Vec<String>,         // LLM生成的标签 (仅对主媒体文件有意义)
    pub purity_score: u8,          // LLM计算的纯净度分数 (仅对主媒体文件有意义)
}

// 注意：BatchLlmFileAnalysis 结构体已被删除，因为未被使用

// --- 3. LLM客户端定义 ---

#[async_trait]
pub trait LlmClient: Send + Sync {
    /// 第一阶段：从HTML页面批量提取基础、原始的磁力链接信息
    async fn batch_extract_basic_info_from_html(
        &self,
        html_content: &str,
        extraction_config: &LlmConfig,
    ) -> Result<BatchExtractBasicInfoResult>;

    /// 第二阶段：根据文件列表批量分析分数和标签
    async fn batch_analyze_scores_and_tags(
        &self,
        original_title: &str,
        file_list: &[String],
        analysis_config: &LlmConfig,
    ) -> Result<(String, u8, Vec<String>)>;
}

pub struct GeminiClient {
    client: Client,
}

impl GeminiClient {
    pub fn new() -> Self {
        let client = Client::new();
        Self { client }
    }
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn batch_extract_basic_info_from_html(
        &self,
        html_content: &str,
        extraction_config: &LlmConfig,
    ) -> Result<BatchExtractBasicInfoResult> {
        self.batch_extract_basic_info_impl(html_content, extraction_config).await
    }

    async fn batch_analyze_scores_and_tags(
        &self,
        original_title: &str,
        file_list: &[String],
        analysis_config: &LlmConfig,
    ) -> Result<(String, u8, Vec<String>)> {
        self.batch_analyze_scores_and_tags_impl(original_title, file_list, analysis_config)
            .await
    }
}

// --- 4. Gemini API请求和响应结构 ---

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

// --- 5. 核心实现 ---

impl GeminiClient {
    /// **第一阶段实现**: 仅从HTML提取原始数据，不做任何修改。
    async fn batch_extract_basic_info_impl(
        &self,
        html_content: &str,
        config: &LlmConfig,
    ) -> Result<BatchExtractBasicInfoResult> {
        let normalized_base = normalize_api_base(&config.api_base);
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            normalized_base, config.model, config.api_key
        );

        let prompt = format!(
            r#"
作为数据提取引擎，你的唯一任务是从以下HTML内容中识别出所有磁力链接条目，并返回一个包含 "results" 数组的JSON对象。

**提取规则:**
1.  **识别条目**: 找到包含磁力链接 (`magnet:?xt=`) 的HTML片段。
2.  **提取字段**:
    *   `title`: 提取与磁力链接相关的最直接的标题文本。**不要进行任何形式的清理、修改或美化**。返回原始文本。
    *   `magnet_link`: 提取完整的磁力链接字符串。
    *   `file_size`: 提取与该条目相关的文件大小文本（例如 "1.5GB", "899MB"）。如果找不到，则返回 `null`。
3.  **严格JSON输出**: 返回的JSON对象必须只包含一个 `results` 键，其值为一个对象数组。每个对象都包含 `title`, `magnet_link`, `file_size` 字段。

**重要指令:**
*   **绝对禁止修改数据**: 你的任务是提取，不是处理。返回你找到的原始信息。
*   **无需理解内容**: 不要尝试理解标题的含义或美化它。
*   **保持顺序**: 尽可能按照在HTML中出现的顺序列出结果。
*   **不要包含任何解释**: 你的输出必须是纯粹的JSON。

**HTML内容:**
```html
{}
```

**示例输出:**
```json
{{
  "results": [
    {{
      "title": "Some.Movie.Title.2023.1080p.BluRay.x264-GROUP[rartv]",
      "magnet_link": "magnet:?xt=urn:btih:abcdef123456...",
      "file_size": "2.3GB"
    }},
    {{
      "title": "[AD] www.example.com [AD] Another.Show.S01E01.720p.WEB-DL",
      "magnet_link": "magnet:?xt=urn:btih:fedcba654321...",
      "file_size": "500MB"
    }}
  ]
}}
```
"#,
            html_content
        );

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let response = self.client.post(&url).json(&request_body).send().await?;
        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API请求失败: {}", error_body));
        }

        let gemini_response = response.json::<GeminiResponse>().await?;
        if let Some(candidate) = gemini_response.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                let cleaned_text = part.text.trim().replace("```json", "").replace("```", "");
                let result: BatchExtractBasicInfoResult = serde_json::from_str(&cleaned_text)
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "解析第一阶段JSON失败: {}. Raw text: {}",
                            e,
                            cleaned_text
                        )
                    })?;
                return Ok(result);
            }
        }
        Err(anyhow::anyhow!("Gemini响应中未找到有效内容"))
    }

    /// **重构后的第二阶段实现**: 根据新的、更简单的逻辑分析标题、文件列表和标签。
    async fn batch_analyze_scores_and_tags_impl(
        &self,
        original_title: &str,
        file_list: &[String],
        config: &LlmConfig,
    ) -> Result<(String, u8, Vec<String>)> {
        let normalized_base = normalize_api_base(&config.api_base);
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            normalized_base, config.model, config.api_key
        );

        let files_json_array = serde_json::to_string(file_list)?;

        let prompt = format!(
            r#"
作为媒体资源分析引擎，请根据以下三项独立任务，对提供的数据进行分析，并严格按照JSON格式返回结果。

**任务1：精简标题**
- **输入**: 原始标题字符串。
- **规则**:
  1. 仅输出作品名称和剧集信息，移除所有其他内容（广告、网址、推广信息、画质、格式等）。
  2. 作品名称：如有多个作品名称或多个语言版本，按英语 → 汉语 → 其他语言的顺序全部输出，用空格分隔。
  3. 剧集信息：如有多个季数或集数，全部输出（如同时有第二季和第三季输出S02 S03，同时有第二季第三集和第一季第二集输出S01E02 S02E03），如原始标题中没有显示则不输出。
  4. 格式：作品名称（多个名称用空格分隔）+ 剧集信息（多个季集用空格分隔），中间用空格分隔。
- **输出**: 返回精简后的标题字符串。

**任务2：计算纯净度分数**
- **输入**: 文件名列表 (JSON Array)。
- **规则**:
  1. 遍历列表中的每个文件名。
  2. 根据以下标准为每个文件打分：
     - **0分**: 纯广告文件（如 `.txt`, `.url`, 或包含明确广告词语的文件）。
     - **80分**: 文件名包含广告信息（如网址）的媒体资源文件。
     - **100分**: 文件名干净、不含任何广告信息的媒体资源文件。
  3. 计算所有文件分数的**平均值**，并四舍五入为整数。
- **输出**: 返回一个0-100之间的整数作为最终纯净度分数。

**任务3：提取标签**
- **输入**: 原始标题字符串。
- **规则**:
  1. **严格按顺序**提取以下4类标签，每类最多1个，总共最多4个标签：
     - **画质**: 使用标准格式（如720p、1080p、4K、8K等）
     - **语言**: 使用英语输出（如Chinese、Korean、Japanese、English等）
     - **字幕**: 按字幕语言输出（如Chinese Sub、English Sub、Korean Sub等）
     - **特殊格式**: 使用英语输出（如BluRay、Dolby、HDR、DV等）
  2. 如果某类信息无法从原始标题中获取，该位置留空，不要编造。
  3. 严格按照上述顺序排列，最多输出4个标签。
- **输出**: 返回包含标签的字符串数组，最多4个元素。

**输入数据:**
- **原始标题**: "{}"
- **文件名列表**: {}

**输出要求:**
- 严格按照以下JSON格式返回，不要包含任何额外的解释或Markdown标记。
- `cleaned_title` 对应任务1的输出。
- `purity_score` 对应任务2的输出。
- `tags` 对应任务3的输出。

**示例输出:**
```json
{{
  "cleaned_title": "Transformers Batman 变形金刚 蝙蝠侠 S01E02 S02E03",
  "purity_score": 95,
  "tags": ["4K", "Chinese", "Chinese Sub", "BluRay"]
}}
```
"#,
            original_title, files_json_array
        );

        // --- 调试输出: 打印最终的Prompt ---
        println!("[AI PROMPT] Full prompt being sent to AI:\n---\n{}\n---", prompt);

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let response = self.client.post(&url).json(&request_body).send().await?;
        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("API请求失败: {}", error_body));
        }

        let gemini_response = response.json::<GeminiResponse>().await?;
        if let Some(candidate) = gemini_response.candidates.get(0) {
            if let Some(part) = candidate.content.parts.get(0) {
                let cleaned_text = part.text.trim().replace("```json", "").replace("```", "");

                // --- 调试输出: 打印原始的AI响应 ---
                println!("[AI RESPONSE] Raw response from AI:\n---\n{}\n---", cleaned_text);
                
                #[derive(Deserialize)]
                struct AnalysisResponse {
                    cleaned_title: String,
                    purity_score: u8,
                    tags: Vec<String>,
                }

                let analysis: AnalysisResponse = serde_json::from_str(&cleaned_text)
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "解析AI响应JSON失败: {}. Raw text: {}",
                            e,
                            cleaned_text
                        )
                    })?;
                
                return Ok((analysis.cleaned_title, analysis.purity_score, analysis.tags));
            }
        }
        Err(anyhow::anyhow!("Gemini响应中未找到有效内容"))
    }
}

// --- 6. 公共API函数 ---
// 注意：原有的公共API函数已被删除，因为它们未被使用
// 所有AI调用现在都通过LlmClient trait进行

/// 测试与LLM提供商的连接。
pub async fn test_connection(config: &LlmConfig) -> Result<String> {
    let normalized_base = normalize_api_base(&config.api_base);
    let url = format!(
        "{}/models/{}:generateContent?key={}",
        normalized_base, config.model, config.api_key
    );

    // 添加调试信息帮助用户诊断问题
    println!("🔧 [连接测试] 原始URL: {}", config.api_base);
    println!("🔧 [连接测试] 标准化URL: {}", normalized_base);
    println!("🔧 [连接测试] 完整请求URL: {}", url);
    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: "你好".to_string(),
            }],
        }],
    };
    let client = Client::new();
    let response = client.post(&url).json(&request_body).send().await?;

    let status = response.status();
    println!("🔧 [连接测试] 响应状态码: {}", status);

    if status.is_success() {
        println!("✅ [连接测试] 连接成功！");
        Ok("连接成功".to_string())
    } else {
        let error_body = response.text().await.unwrap_or_default();
        println!("❌ [连接测试] 错误响应: {}", error_body);

        // 为常见错误提供更友好的提示
        let error_message = match status.as_u16() {
            401 => "认证失败：请检查API Key是否正确".to_string(),
            403 => "访问被拒绝：请检查API Key权限".to_string(),
            404 => "API路径不存在：请检查API Base URL是否正确".to_string(),
            405 => "请求方法不允许：API路径可能不正确".to_string(),
            500 => "服务器内部错误：可能是API Key无效或模型名称错误".to_string(),
            _ => format!("API连接失败 (状态码: {})", status),
        };

        Err(anyhow::anyhow!("{}: {}", error_message, error_body))
    }
}