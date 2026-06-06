use anyhow::{Result, anyhow};
// 移除未使用的顶层导入（reqwest 已通过具体路径使用）
use scraper::{Html, Selector};
use futures::{
    future::join_all,
    stream::{self, StreamExt},
};
use std::sync::Arc;
use crate::llm_service::{LlmClient, LlmConfig, OpenAiCompatibleClient};

// 统一的日志宏
macro_rules! search_log {
    (info, $($arg:tt)*) => {
        println!("🔍 {}", format!($($arg)*))
    };
    (success, $($arg:tt)*) => {
        println!("✅ {}", format!($($arg)*))
    };
    (warn, $($arg:tt)*) => {
        println!("⚠️ {}", format!($($arg)*))
    };
    (error, $($arg:tt)*) => {
        println!("❌ {}", format!($($arg)*))
    };
    (ai, $($arg:tt)*) => {
        println!("🤖 {}", format!($($arg)*))
    };
    (stats, $($arg:tt)*) => {
        println!("📊 {}", format!($($arg)*))
    };
}

// 统一的错误处理
fn handle_request_error(url: &str, error: reqwest::Error) -> anyhow::Error {
    search_log!(error, "Request failed for {}: {}", url, error);
    anyhow!("Request failed: {}", error)
}

/// 安全截断字符串，避免切到多字节字符中间
fn safe_truncate(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }

    // 找到不超过max_bytes的最大字符边界
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// 清理HTML标签和实体
fn clean_html_text(text: &str) -> String {
    // 移除HTML标签
    let re_tags = regex::Regex::new(r"<[^>]*>").unwrap();
    let text = re_tags.replace_all(text, "");

    // 解码常见的HTML实体
    let text = text
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");

    // 清理多余的空格
    text.trim().replace("  ", " ")
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub magnet_link: String,
    pub file_size: Option<String>,
    pub upload_date: Option<String>,
    pub file_list: Vec<String>,
    pub source_url: Option<String>,
    pub score: Option<u8>,
    pub tags: Option<Vec<String>>,
}

/// 搜索引擎提供商特性
#[async_trait::async_trait]
pub trait SearchProvider: Send + Sync {
    #[allow(dead_code)]
    fn name(&self) -> &str;
    async fn search(&self, query: &str, page: u32) -> Result<Vec<SearchResult>>;
}

#[derive(Debug, Clone)]
pub struct SearchEngineConfig {
    pub name: String,
    pub url: String,
    pub engine_type: String,
    pub api_key: String,
    pub categories: Vec<u32>,
    pub allow_adult: bool,
}

pub struct ProwlarrProvider {
    name: String,
    base_url: String,
    api_key: String,
    categories: Vec<u32>,
    allow_adult: bool,
    client: reqwest::Client,
}

impl ProwlarrProvider {
    pub fn new(config: SearchEngineConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("Failed to create Prowlarr HTTP client");
        Self {
            name: config.name,
            base_url: config.url.trim_end_matches('/').to_string(),
            api_key: config.api_key,
            categories: config.categories,
            allow_adult: config.allow_adult,
            client,
        }
    }

    async fn enabled_indexer_ids(&self) -> Result<Vec<u32>> {
        let response = self
            .client
            .get(format!("{}/api/v1/indexer", self.base_url))
            .header("X-Api-Key", self.api_key.trim())
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(anyhow!("Prowlarr indexer list failed (HTTP {status})"));
        }

        let indexers: Vec<ProwlarrIndexer> = response.json().await?;
        Ok(indexers
            .into_iter()
            .filter(|indexer| indexer.enable)
            .map(|indexer| indexer.id)
            .collect())
    }

    async fn search_indexer(
        &self,
        query: &str,
        page: u32,
        indexer_id: Option<u32>,
    ) -> Result<Vec<SearchResult>> {
        let mut request = self
            .client
            .get(format!("{}/api/v1/search", self.base_url))
            .header("X-Api-Key", self.api_key.trim())
            .query(&[
                ("query", query.to_string()),
                ("type", "search".to_string()),
                ("limit", "100".to_string()),
                ("offset", ((page.saturating_sub(1)) * 100).to_string()),
            ]);
        if let Some(indexer_id) = indexer_id {
            request = request.query(&[("indexerIds", indexer_id.to_string())]);
        }
        for category in &self.categories {
            request = request.query(&[("categories", category)]);
        }

        let response = tokio::time::timeout(std::time::Duration::from_secs(6), request.send())
            .await
            .map_err(|_| anyhow!("Prowlarr indexer search timed out"))??;
        let status = response.status();
        if !status.is_success() {
            return Err(anyhow!("Prowlarr search failed (HTTP {status})"));
        }

        let releases: Vec<ProwlarrRelease> = response.json().await?;
        Ok(self.releases_to_results(releases))
    }

    fn releases_to_results(&self, releases: Vec<ProwlarrRelease>) -> Vec<SearchResult> {
        releases
            .into_iter()
            .filter(|release| {
                self.allow_adult
                    || !release
                        .categories
                        .iter()
                        .any(|category| (6000..7000).contains(&category.id))
            })
            .filter_map(|release| {
                let magnet_link = release
                    .magnet_url
                    .filter(|url| url.starts_with("magnet:?"))
                    .or_else(|| {
                        release
                            .info_hash
                            .filter(|hash| !hash.trim().is_empty())
                            .map(|hash| format!("magnet:?xt=urn:btih:{hash}"))
                    })?;
                Some(SearchResult {
                    title: release.title.unwrap_or_else(|| "Unknown".to_string()),
                    magnet_link,
                    file_size: release.size.map(format_bytes),
                    upload_date: release.publish_date,
                    file_list: Vec::new(),
                    source_url: release.info_url.or(release.comment_url),
                    score: None,
                    tags: None,
                })
            })
            .collect()
    }
}

#[derive(serde::Deserialize)]
struct ProwlarrIndexer {
    id: u32,
    #[serde(default)]
    enable: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProwlarrRelease {
    title: Option<String>,
    size: Option<u64>,
    publish_date: Option<String>,
    info_url: Option<String>,
    comment_url: Option<String>,
    magnet_url: Option<String>,
    info_hash: Option<String>,
    #[serde(default)]
    categories: Vec<ProwlarrCategory>,
}

#[derive(serde::Deserialize)]
struct ProwlarrCategory {
    id: u32,
}

#[async_trait::async_trait]
impl SearchProvider for ProwlarrProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn search(&self, query: &str, page: u32) -> Result<Vec<SearchResult>> {
        if self.api_key.trim().is_empty() {
            return Err(anyhow!("Prowlarr API key is required"));
        }

        let indexer_ids = self.enabled_indexer_ids().await.unwrap_or_default();
        if indexer_ids.is_empty() {
            return self.search_indexer(query, page, None).await;
        }

        let results = stream::iter(indexer_ids)
            .map(|indexer_id| async move { self.search_indexer(query, page, Some(indexer_id)).await })
            .buffer_unordered(16)
            .collect::<Vec<_>>()
            .await;

        let mut all_results = Vec::new();
        let mut successful_indexers = 0usize;
        let mut errors = Vec::new();
        for result in results {
            match result {
                Ok(mut indexer_results) => {
                    successful_indexers += 1;
                    all_results.append(&mut indexer_results);
                }
                Err(error) => errors.push(error.to_string()),
            }
        }

        if successful_indexers == 0 && !errors.is_empty() {
            return Err(anyhow!("All Prowlarr indexers failed: {}", errors.join("; ")));
        }
        Ok(all_results)
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.2} {}", UNITS[unit])
}

/// 通用搜索引擎提供商，支持自定义URL模板和AI智能识别
pub struct GenericProvider {
    name: String,
    url_template: String,
    client: reqwest::Client,
    llm_client: Option<Arc<dyn LlmClient>>,
    extraction_config: Option<LlmConfig>,  // HTML提取配置（分析由前端处理）
    priority_keywords: Vec<String>,
}

impl GenericProvider {
    pub fn new(name: String, url_template: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            // reqwest默认启用gzip/deflate解压，不需要显式设置
            .build()
            .expect("Failed to create HTTP client");

        Self {
            name,
            url_template,
            client,
            llm_client: None,
            extraction_config: None,
            priority_keywords: Vec::new(),
        }
    }

    /// 设置 LLM 客户端和（第一阶段 HTML 提取用的）配置
    pub fn with_llm_client_and_config(
        mut self,
        llm_client: Arc<dyn LlmClient>,
        extraction_config: LlmConfig,
    ) -> Self {
        self.llm_client = Some(llm_client);
        self.extraction_config = Some(extraction_config);
        self
    }

    /// 设置优先关键词用于匹配
    pub fn with_priority_keywords(mut self, keywords: Vec<String>) -> Self {
        self.priority_keywords = keywords;
        self
    }
}

#[async_trait::async_trait]
impl SearchProvider for GenericProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn search(&self, query: &str, page: u32) -> Result<Vec<SearchResult>> {
        // 替换URL模板中的占位符
        let encoded_query = urlencoding::encode(query);
        let mut url = self.url_template
            .replace("{keyword}", encoded_query.as_ref());

        // Handle different page numbering systems
        if url.contains("{page-1}") {
            // 0-based pagination: subtract 1 from page number
            let zero_based_page = if page > 0 { page - 1 } else { 0 };
            url = url.replace("{page-1}", &zero_based_page.to_string());
        } else {
            // 1-based pagination (default)
            url = url.replace("{page}", &page.to_string());
        }

        search_log!(info, "Searching: {}", url);

        let response = self.client
            .get(&url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Sec-Ch-Ua", "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"")
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("Sec-Ch-Ua-Platform", "\"Windows\"")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "cross-site")
            .header("Sec-Fetch-User", "?1")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Referer", "https://www.google.com/")
            .send()
            .await
            .map_err(|e| handle_request_error(&url, e))?;

        if !response.status().is_success() {
            search_log!(error, "HTTP error {} for {}", response.status(), url);
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        // 获取响应文本（reqwest自动处理压缩）
        let html = response.text().await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        // 检查响应内容类型
        let is_javascript = html.trim_start().starts_with("\"use strict\"") ||
                           html.contains("webpack") ||
                           html.contains("self.webpackChunk");

        if is_javascript {
            search_log!(warn, "网站返回JavaScript代码，可能是SPA或有反爬虫机制，跳过处理");
            return Ok(Vec::new());
        }

        if html.contains('�') {
            search_log!(warn, "HTML包含乱码字符，可能存在编码问题");
        }

        // 只在出现问题时显示HTML预览
        if html.contains('�') || is_javascript {
            let preview = safe_truncate(&html, 500);
            search_log!(info, "HTML preview (前500字符，用于诊断):");
            println!("---START---");
            println!("{preview}");
            println!("---END---");
        }

        // 简单检查内容
        let magnet_count = html.matches("magnet:").count();
        if magnet_count == 0 {
            let error_count = html.matches("404").count() + html.matches("Not Found").count();
            if error_count > 0 {
                search_log!(warn, "可能收到了错误页面，包含 {} 个错误指示符", error_count);
            }
        }

        // 对于自定义搜索引擎，使用AI智能识别流程
        let results = if let Some(llm_client) = &self.llm_client {
            self.analyze_html_with_ai(&html, llm_client.clone()).await?
        } else {
            self.parse_generic_results(&html)?
        };

        search_log!(stats, "Found {} results on page {}", results.len(), page);
        Ok(results)
    }
}

impl GenericProvider {
    /// 使用AI分析整个HTML内容
    async fn analyze_html_with_ai(&self, html: &str, llm_client: Arc<dyn LlmClient>) -> Result<Vec<SearchResult>> {
        search_log!(ai, "Phase 1: Extracting basic info from HTML...");

        // 第一阶段：让AI从HTML中提取所有磁力链接和基础信息
        match self.extract_torrents_from_html_with_ai(html, llm_client.clone()).await {
            Ok(results) => {
                if results.is_empty() {
                    search_log!(warn, "AI extraction found no results. Falling back to basic parsing");
                    return self.parse_generic_results(html);
                }

                search_log!(ai, "Phase 2: Separating priority results...");
                let (priority_results, regular_results) = self.separate_priority_results(results);

                search_log!(success, "AI extraction completed: {} priority and {} regular results",
                         priority_results.len(), regular_results.len());

                // 合并结果：优先结果在前，普通结果在后
                let mut final_results = priority_results;
                final_results.extend(regular_results);
                Ok(final_results)
            }
            Err(e) => {
                search_log!(warn, "AI HTML analysis failed: {}, falling back to basic parsing", e);
                self.parse_generic_results(html)
            }
        }
    }

    /// 使用AI从HTML中提取种子信息
    async fn extract_torrents_from_html_with_ai(&self, html: &str, llm_client: Arc<dyn LlmClient>) -> Result<Vec<SearchResult>> {
        // 限制HTML长度以避免超出AI token限制 (250k tokens模型，使用80k字符约120k tokens)
        let truncated_html = if html.len() > 80000 {
            search_log!(info, "HTML too long ({} chars), truncating to 80k chars", html.len());
            safe_truncate(html, 80000)
        } else {
            html
        };

        // 直接传递原始HTML给AI服务，让llm_service.rs负责构建提示词
        match self.call_ai_for_html_analysis(truncated_html, llm_client).await {
            Ok(ai_results) => Ok(ai_results),
            Err(e) => Err(anyhow!("AI HTML analysis failed: {}", e))
        }
    }

    /// 直接调用AI进行HTML分析
    async fn call_ai_for_html_analysis(&self, html_content: &str, llm_client: Arc<dyn LlmClient>) -> Result<Vec<SearchResult>> {
        // 获取提取配置
        let extraction_config = self.extraction_config.as_ref()
            .ok_or_else(|| anyhow!("Extraction config not available"))?;

        // 将原始HTML传递给AI服务，由llm_service.rs构建提示词
        match llm_client.batch_extract_basic_info_from_html(html_content, extraction_config).await {
            Ok(batch_result) => {
                // AI返回的JSON响应被解析到batch_result.results中
                // 我们需要将整个结果传递给解析函数
                self.parse_ai_html_response_from_batch(batch_result)
            }
            Err(e) => {
                search_log!(error, "AI HTML分析失败: {}", e);
                search_log!(ai, "发送给AI的HTML长度: {} 字符", html_content.len());
                search_log!(ai, "HTML前500字符预览: {}", safe_truncate(html_content, 500));
                Err(anyhow!("AI HTML analysis failed: {}", e))
            }
        }
    }

    /// 解析AI返回的HTML分析结果
    fn parse_ai_html_response_from_batch(&self, batch_result: crate::llm_service::BatchExtractBasicInfoResult) -> Result<Vec<SearchResult>> {
        // 直接从BatchExtractBasicInfoResult转换为SearchResult
        let mut results = Vec::new();

        for basic_info in batch_result.results {
            // 验证磁力链接格式
            if !basic_info.magnet_link.starts_with("magnet:?xt=urn:btih:") {
                println!("⚠️ Invalid magnet link format, skipping: {}", basic_info.magnet_link);
                continue;
            }

            // 处理 source_url：统一使用 normalize_source_url
            let source_url = basic_info
                .source_url
                .map(|href| self.normalize_source_url(&href));

            results.push(SearchResult {
                title: clean_html_text(&basic_info.title),
                magnet_link: basic_info.magnet_link,
                file_size: basic_info.file_size,
                upload_date: None, // 第一阶段不提取上传日期
                file_list: Vec::new(),
                source_url,
                score: None,
                tags: None,
            });
        }

        Ok(results)
    }

    /// 从URL模板中提取基础URL（用于构建完整的source_url）
    fn extract_base_url_from_template(&self) -> Option<String> {
        if let Ok(parsed_url) = url::Url::parse(&self.url_template) {
            if let Some(host) = parsed_url.host_str() {
                let scheme = parsed_url.scheme();
                return Some(format!("{scheme}://{host}"));
            }
        }
        None
    }

    /// 标准化source_url，将相对路径转换为绝对路径
    fn normalize_source_url(&self, href: &str) -> String {
        if href.starts_with("http://") || href.starts_with("https://") {
            href.to_string()
        } else if href.starts_with("/") {
            // 相对路径，需要从URL模板中提取基础域名
            self.extract_base_url_from_template()
                .map(|base| format!("{base}{href}"))
                .unwrap_or_else(|| href.to_string())
        } else {
            href.to_string()
        }
    }

    // 注意：parse_ai_html_response 函数已被删除，因为现在直接使用 BatchExtractBasicInfoResult

    /// 分离优先结果和普通结果
    fn separate_priority_results(&self, results: Vec<SearchResult>) -> (Vec<SearchResult>, Vec<SearchResult>) {
        if self.priority_keywords.is_empty() {
            return (Vec::new(), results);
        }

        let (priority_results, regular_results): (Vec<_>, Vec<_>) = results.into_iter().partition(|result| {
            let title_lower = result.title.to_lowercase();
            self.priority_keywords.iter().any(|keyword| title_lower.contains(&keyword.to_lowercase()))
        });

        if !priority_results.is_empty() {
            println!("🌟 Found {} priority results.", priority_results.len());
        }

        (priority_results, regular_results)
    }

    // 注意：apply_detailed_ai_analysis 方法已被移除
    // 现在统一使用前端的并行分析流程，提供更好的用户体验

    fn parse_generic_results(&self, html: &str) -> Result<Vec<SearchResult>> {
        let document = Html::parse_document(html);
        let mut results = Vec::new();

        println!("🔍 Parsing generic HTML content...");

        // 尝试查找常见的磁力链接模式
        let magnet_regex = regex::Regex::new(r"magnet:\?xt=urn:btih:[a-fA-F0-9]{40}[^&\s]*")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;

        // 尝试解析表格结构（最常见的种子站点布局）
        if let Ok(table_selector) = Selector::parse("table") {
            for table in document.select(&table_selector) {
                if let Ok(row_selector) = Selector::parse("tr") {
                    for row in table.select(&row_selector) {
                        if let Some(result) = self.parse_table_row(&row, &magnet_regex) {
                            results.push(result);
                        }
                    }
                }
            }
        }

        // 如果表格解析没有结果，尝试通用解析
        if results.is_empty() {
            results = self.parse_generic_fallback(&document, &magnet_regex)?;
        }

        println!("📊 Extracted {} unique results from generic HTML", results.len());
        Ok(results)
    }

    /// 解析表格行，提取标题、磁力链接和文件大小
    fn parse_table_row(&self, row: &scraper::ElementRef, magnet_regex: &regex::Regex) -> Option<SearchResult> {
        let row_html = row.html();

        // 查找磁力链接
        let magnet_link = magnet_regex.find(&row_html)?.as_str().to_string();

        // 提取单元格
        let cell_selector = Selector::parse("td").ok()?;
        let cells: Vec<_> = row.select(&cell_selector).collect();

        if cells.is_empty() {
            return None;
        }

        let mut title = None;
        let mut file_size = None;
        let mut upload_date = None;
        let mut source_url = None;

        // 分析每个单元格
        for (i, cell) in cells.iter().enumerate() {
            let cell_text = cell.text().collect::<String>().trim().to_string();

            // 第一个单元格通常是标题
            if i == 0 && title.is_none() {
                if let Ok(link_selector) = Selector::parse("a") {
                    if let Some(link) = cell.select(&link_selector).next() {
                        let link_text = link.text().collect::<String>().trim().to_string();
                        if !link_text.is_empty() && !link_text.starts_with("magnet:") {
                            title = Some(clean_html_text(&link_text));
                            // 提取source_url
                            if let Some(href) = link.value().attr("href") {
                                source_url = Some(self.normalize_source_url(href));
                            }
                        }
                    }
                }
                // 如果没有链接，使用单元格文本
                if title.is_none() && !cell_text.is_empty() && cell_text.len() > 5 {
                    title = Some(clean_html_text(&cell_text));
                }
            }

            // 查找文件大小（包含 GB, MB, KB, TB 的单元格）
            if file_size.is_none() && self.is_file_size(&cell_text) {
                file_size = Some(cell_text.clone());
            }

            // 查找日期（包含日期格式的单元格）
            if upload_date.is_none() && self.is_date(&cell_text) {
                upload_date = Some(cell_text);
            }
        }

        // 如果没有找到标题，尝试从磁力链接提取
        let final_title = title.unwrap_or_else(|| self.extract_title_from_magnet(&magnet_link));

        Some(SearchResult {
            title: final_title,
            magnet_link,
            file_size,
            upload_date,
            file_list: Vec::new(),
            source_url,
            score: None,
            tags: None,
        })
    }

    /// 通用回退解析方法
    fn parse_generic_fallback(&self, document: &Html, magnet_regex: &regex::Regex) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let mut seen_magnets = std::collections::HashSet::new();

        for magnet_match in magnet_regex.find_iter(&document.html()) {
            let magnet_link = magnet_match.as_str();

            if seen_magnets.insert(magnet_link.to_string()) {
                let title = self.extract_title_from_magnet(magnet_link);
                results.push(SearchResult {
                    title,
                    magnet_link: magnet_link.to_string(),
                    file_size: None,
                    upload_date: None,
                    file_list: Vec::new(),
                    source_url: None,
                    score: None,
                    tags: None,
                });
            }
        }

        Ok(results)
    }

    /// 判断文本是否是文件大小
    fn is_file_size(&self, text: &str) -> bool {
        let text_upper = text.to_uppercase();
        (text_upper.contains("GB") || text_upper.contains("MB") ||
         text_upper.contains("KB") || text_upper.contains("TB")) &&
        text.chars().any(|c| c.is_ascii_digit())
    }

    /// 判断文本是否是日期
    fn is_date(&self, text: &str) -> bool {
        // 简单的日期格式检测
        text.contains("-") && text.len() >= 8 && text.len() <= 20 &&
        text.chars().filter(|c| c.is_ascii_digit()).count() >= 4
    }

    /// 从磁力链接的dn参数中提取标题
    fn extract_title_from_magnet(&self, magnet_link: &str) -> String {
        // 尝试从磁力链接的dn参数中提取文件名
        if let Some(dn_start) = magnet_link.find("&dn=") {
            let dn_part = &magnet_link[dn_start + 4..];
            if let Some(dn_end) = dn_part.find('&') {
                let dn_value = &dn_part[..dn_end];
                // URL解码
                if let Ok(decoded) = urlencoding::decode(dn_value) {
                    let decoded_str = decoded.to_string();
                    if !decoded_str.is_empty() && decoded_str.len() > 5 {
                        return decoded_str;
                    }
                }
            } else {
                // dn是最后一个参数
                if let Ok(decoded) = urlencoding::decode(dn_part) {
                    let decoded_str = decoded.to_string();
                    if !decoded_str.is_empty() && decoded_str.len() > 5 {
                        return decoded_str;
                    }
                }
            }
        }

        // 如果无法从dn参数提取，生成一个基于哈希的标题
        let hash_part = if let Some(btih_start) = magnet_link.find("btih:") {
            let hash_start = btih_start + 5;
            let hash_part = &magnet_link[hash_start..];
            if let Some(hash_end) = hash_part.find('&') {
                &hash_part[..hash_end.min(8)]
            } else {
                &hash_part[..8.min(hash_part.len())]
            }
        } else {
            "unknown"
        };

        format!("Torrent_{hash_part}")
    }
}

/// 搜索引擎核心
pub struct SearchCore {
    providers: Vec<Arc<dyn SearchProvider>>,
}

impl SearchCore {
    // 注意：基础构造函数已被删除，统一使用 create_ai_enhanced_search_core

    /// 并发搜索所有提供商和请求页
    pub async fn search_multi_page(&self, query: &str, max_pages: u32) -> Result<Vec<SearchResult>> {
        if self.providers.is_empty() {
            return Err(anyhow!("No search providers available"));
        }

        println!("🔍 Starting search with {} providers, {} pages each", self.providers.len(), max_pages);

        let mut search_futures = Vec::new();
        for provider in &self.providers {
            for page in 1..=max_pages {
                let provider = Arc::clone(provider);
                let query = query.to_string();
                let provider_name = provider.name().to_string();
                search_futures.push(async move {
                    println!("🔍 Searching {query} page {page} with provider: {provider_name}");
                    provider.search(&query, page).await
                });
            }
        }

        let mut all_results = Vec::new();
        let mut successful_requests = 0usize;
        let mut errors = Vec::new();
        for result in join_all(search_futures).await {
            match result {
                Ok(mut page_results) => {
                    successful_requests += 1;
                    all_results.append(&mut page_results);
                }
                Err(e) => {
                    println!("⚠️ Search task failed: {e}");
                    errors.push(e.to_string());
                }
            }
        }

        if successful_requests == 0 && !errors.is_empty() {
            return Err(anyhow!("All search engines failed: {}", errors.join("; ")));
        }

        let results = deduplicate_by_btih(all_results);
        println!("🎯 Total unique results collected from all providers: {}", results.len());
        Ok(results)
    }



    /// 单页搜索（向后兼容）
    #[allow(dead_code)]
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        self.search_multi_page(query, 1).await
    }
}

/// 创建带有AI功能的搜索核心
pub fn create_ai_enhanced_search_core(
    extraction_config: Option<LlmConfig>,
    analysis_config: Option<LlmConfig>, // 保持向后兼容，但现在只用于HTML提取
    priority_keywords: Vec<String>,
    custom_engines: Vec<SearchEngineConfig>,
) -> SearchCore {
    let mut providers: Vec<Arc<dyn SearchProvider>> = Vec::new();

    // 为自定义搜索引擎创建AI增强的提供商
    // 优先使用 extraction_config，如果没有则使用 analysis_config（向后兼容）
    let html_extraction_config = extraction_config.or(analysis_config);

    if let Some(extract_config) = html_extraction_config {
        let llm_client: Arc<dyn LlmClient> = Arc::new(OpenAiCompatibleClient::new());

        for engine in custom_engines {
            if engine.engine_type == "prowlarr" {
                println!("✅ Adding Prowlarr provider: {}", engine.name);
                providers.push(Arc::new(ProwlarrProvider::new(engine)));
            } else {
                println!("✅ Adding AI-enhanced custom provider: {}", engine.name);
                let provider = GenericProvider::new(engine.name, engine.url)
                    .with_llm_client_and_config(llm_client.clone(), extract_config.clone())
                    .with_priority_keywords(priority_keywords.clone());
                providers.push(Arc::new(provider));
            }
        }
    } else {
        // 如果没有LLM配置，创建基础的自定义提供商
        for engine in custom_engines {
            if engine.engine_type == "prowlarr" {
                println!("✅ Adding Prowlarr provider: {}", engine.name);
                providers.push(Arc::new(ProwlarrProvider::new(engine)));
            } else {
                println!("✅ Adding basic custom provider: {}", engine.name);
                providers.push(Arc::new(GenericProvider::new(engine.name, engine.url)));
            }
        }
    }

    SearchCore { providers }
}



#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    // removed redundant single-component import per clippy

    struct FailingProvider;

    #[async_trait::async_trait]
    impl SearchProvider for FailingProvider {
        fn name(&self) -> &str {
            "failing"
        }

        async fn search(&self, _query: &str, _page: u32) -> Result<Vec<SearchResult>> {
            Err(anyhow!("unauthorized"))
        }
    }

    #[tokio::test]
    async fn reports_error_when_all_search_engines_fail() {
        let core = SearchCore {
            providers: vec![Arc::new(FailingProvider)],
        };
        let error = core.search_multi_page("test", 1).await.unwrap_err();
        assert!(error.to_string().contains("unauthorized"));
    }

    #[tokio::test]
    async fn prowlarr_search_uses_api_key_and_filters_adult_categories() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/v1/search")
                .header("x-api-key", "secret")
                .query_param("query", "test")
                .query_param("type", "search");
            then.status(200).json_body_obj(&serde_json::json!([
                {
                    "title": "Allowed",
                    "magnetUrl": "http://127.0.0.1:9696/1/download?apikey=secret",
                    "infoHash": "allowed",
                    "size": 1073741824,
                    "categories": [{"id": 2000}]
                },
                {
                    "title": "Filtered",
                    "infoHash": "adult",
                    "categories": [{"id": 6000}]
                }
            ]));
        });

        let provider = ProwlarrProvider::new(SearchEngineConfig {
            name: "Prowlarr".to_string(),
            url: server.base_url(),
            engine_type: "prowlarr".to_string(),
            api_key: "secret".to_string(),
            categories: Vec::new(),
            allow_adult: false,
        });
        let results = provider.search("test", 1).await.unwrap();

        mock.assert();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Allowed");
        assert_eq!(results[0].magnet_link, "magnet:?xt=urn:btih:allowed");
        assert_eq!(results[0].file_size.as_deref(), Some("1.00 GB"));
    }

    #[test]
    fn deduplicates_results_by_btih_case_insensitively() {
        let make_result = |magnet_link: &str| SearchResult {
            title: "Title".to_string(),
            magnet_link: magnet_link.to_string(),
            file_size: None,
            upload_date: None,
            file_list: Vec::new(),
            source_url: None,
            score: None,
            tags: None,
        };
        let results = deduplicate_by_btih(vec![
            make_result("magnet:?xt=urn:btih:ABC123&dn=one"),
            make_result("magnet:?xt=urn:btih:abc123&dn=two"),
        ]);
        assert_eq!(results.len(), 1);
    }
}

fn deduplicate_by_btih(results: Vec<SearchResult>) -> Vec<SearchResult> {
    let mut seen = std::collections::HashSet::new();
    results
        .into_iter()
        .filter(|result| {
            let key = btih_key(&result.magnet_link).unwrap_or_else(|| result.magnet_link.to_lowercase());
            seen.insert(key)
        })
        .collect()
}

fn btih_key(magnet_link: &str) -> Option<String> {
    let lower = magnet_link.to_lowercase();
    let start = lower.find("urn:btih:")? + "urn:btih:".len();
    let hash = lower[start..].split('&').next()?.trim();
    (!hash.is_empty()).then(|| hash.to_string())
}
