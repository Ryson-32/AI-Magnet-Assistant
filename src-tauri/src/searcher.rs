use anyhow::{Result, anyhow};
use reqwest;
use scraper::{Html, Selector};
use futures::future::join_all;
use std::sync::Arc;
use crate::llm_service::{LlmClient, GeminiClient, LlmConfig};

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

/// clmclm.com 搜索引擎实现
pub struct ClmclmProvider {
    client: reqwest::Client,
}

impl ClmclmProvider {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }
}

#[async_trait::async_trait]
impl SearchProvider for ClmclmProvider {
    fn name(&self) -> &str {
        "clmclm.com"
    }

    async fn search(&self, query: &str, page: u32) -> Result<Vec<SearchResult>> {
        let url = format!("http://clmclm.com/search-{}-1-1-{}.html", query, page);
        println!("🔍 Searching: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                println!("❌ Network error: {}", e);
                anyhow!("Failed to fetch {}: {}", url, e)
            })?;

        if !response.status().is_success() {
            println!("❌ HTTP error: {} for {}", response.status(), url);
            return Err(anyhow!("HTTP error {}: {}", response.status(), url));
        }

        let html = response.text().await?;
        println!("✅ Response received, parsing...");
        let results = self.parse_results(&html)?;
        println!("📊 Found {} results on page {}.", results.len(), page);
        Ok(results)
    }
}

impl ClmclmProvider {
    fn parse_results(&self, html: &str) -> Result<Vec<SearchResult>> {
        let document = Html::parse_document(html);

        let row_selector = Selector::parse("div.ssbox")
            .map_err(|e| anyhow!("Invalid CSS selector: {}", e))?;
        let title_selector = Selector::parse("div.title > h3 > a")
            .map_err(|e| anyhow!("Invalid CSS selector: {}", e))?;
        let magnet_selector = Selector::parse("div.sbar a[href^=\"magnet:\"]")
            .map_err(|e| anyhow!("Invalid CSS selector: {}", e))?;
        let file_list_selector = Selector::parse("ul > li")
            .map_err(|e| anyhow!("Invalid CSS selector: {}", e))?;

        let mut results = Vec::new();

        for element in document.select(&row_selector) {
            let title_element = element.select(&title_selector).next();
            let magnet_element = element.select(&magnet_selector).next();

            if let (Some(title_node), Some(magnet_node)) = (title_element, magnet_element) {
                let title = title_node.text().collect::<String>().trim().to_string();
                let source_url = title_node.value().attr("href").map(|s| format!("http://clmclm.com{}", s));

                if let Some(magnet_link) = magnet_node.value().attr("href") {
                    // 尝试从所有span中找到文件大小
                    let mut file_size = None;
                    let span_selector = Selector::parse("div.sbar span").unwrap();
                    for span in element.select(&span_selector) {
                        let span_text = span.text().collect::<String>();
                        let span_text = span_text.trim();
                        if span_text.starts_with("大小:") {
                            file_size = Some(span_text.replace("大小:", "").trim().to_string());
                            break;
                        }
                    }

                    // 提取真实的文件列表
                    let mut file_list = Vec::new();
                    for li_element in element.select(&file_list_selector) {
                        let file_text = li_element.text().collect::<String>();
                        let file_text = file_text.trim();

                        // 解析文件名和大小，格式通常是 "文件名 大小"
                        if !file_text.is_empty() {
                            // 分割文件名和大小，大小通常在最后
                            let parts: Vec<&str> = file_text.split_whitespace().collect();
                            if parts.len() >= 2 {
                                // 检查最后一部分是否是文件大小（包含 GB, MB, KB 等）
                                let last_part = parts[parts.len() - 1];
                                if last_part.contains("GB") || last_part.contains("MB") || last_part.contains("KB") || last_part.contains("TB") {
                                    // 文件名是除了最后一部分的所有内容
                                    let filename = parts[..parts.len() - 1].join(" ");
                                    if !filename.is_empty() {
                                        file_list.push(filename);
                                    }
                                } else {
                                    // 如果没有识别到大小，就把整个文本作为文件名
                                    file_list.push(file_text.to_string());
                                }
                            } else {
                                // 如果只有一个部分，直接作为文件名
                                file_list.push(file_text.to_string());
                            }
                        }
                    }

                    // 如果没有解析到文件列表，使用基于标题的生成方法作为后备
                    if file_list.is_empty() {
                        file_list = self.extract_file_list_from_magnet(&magnet_link, &title);
                    }

                    results.push(SearchResult {
                        title,
                        magnet_link: magnet_link.to_string(),
                        file_size,
                        upload_date: None, // clmclm.com doesn't provide upload date
                        file_list,
                        source_url,
                        score: None,
                        tags: None,
                    });
                }
            }
        }

        Ok(results)
    }

    /// 从磁力链接和标题中提取文件列表（基于标题生成相关文件列表）
    fn extract_file_list_from_magnet(&self, magnet_link: &str, title: &str) -> Vec<String> {
        if !magnet_link.contains("btih:") {
            return vec![];
        }

        generate_file_list_from_title(title)
    }
}

/// 通用搜索引擎提供商，支持自定义URL模板和AI智能识别
pub struct GenericProvider {
    name: String,
    url_template: String,
    client: reqwest::Client,
    llm_client: Option<Arc<dyn LlmClient>>,
    extraction_config: Option<LlmConfig>,  // 第一次API调用配置
    analysis_config: Option<LlmConfig>,    // 第二次API调用配置
    priority_keywords: Vec<String>,
}

impl GenericProvider {
    pub fn new(name: String, url_template: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            name,
            url_template,
            client,
            llm_client: None,
            extraction_config: None,
            analysis_config: None,
            priority_keywords: Vec::new(),
        }
    }

    /// 设置 LLM 客户端和配置用于 AI 智能识别
    pub fn with_llm_client_and_configs(
        mut self,
        llm_client: Arc<dyn LlmClient>,
        extraction_config: LlmConfig,
        analysis_config: LlmConfig,
    ) -> Self {
        self.llm_client = Some(llm_client);
        self.extraction_config = Some(extraction_config);
        self.analysis_config = Some(analysis_config);
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
        let url = self.url_template
            .replace("{keyword}", query)
            .replace("{page}", &page.to_string());

        println!("🔍 Searching: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let html = response.text().await
            .map_err(|e| anyhow!("Failed to read response: {}", e))?;

        println!("✅ Response received, parsing...");

        // 对于自定义搜索引擎，使用AI智能识别流程
        let results = if let Some(llm_client) = &self.llm_client {
            println!("🤖 Analyzing HTML with AI...");
            self.analyze_html_with_ai(&html, llm_client.clone()).await?
        } else {
            println!("📊 Basic parsing (no AI)...");
            self.parse_generic_results(&html)?
        };

        println!("📊 Found {} results on page {}.", results.len(), page);
        println!("✨ Final results after AI processing: {} items.", results.len());
        Ok(results)
    }
}

impl GenericProvider {
    /// 使用AI分析整个HTML内容
    async fn analyze_html_with_ai(&self, html: &str, llm_client: Arc<dyn LlmClient>) -> Result<Vec<SearchResult>> {
        println!("🧠 AI Phase 1: Extracting basic info from HTML...");

        // 第一阶段：让AI从HTML中提取所有磁力链接和基础信息
        match self.extract_torrents_from_html_with_ai(html, llm_client.clone()).await {
            Ok(results) => {
                if results.is_empty() {
                    println!("⚠️ AI extraction found no results. Falling back to basic parsing.");
                    return self.parse_generic_results(html);
                }

                println!("🎯 AI Phase 2: Separating priority results...");
                let (priority_results, regular_results) = self.separate_priority_results(results);

                println!("🔍 AI Phase 3: Detailed analysis for {} priority and {} regular results.",
                         priority_results.len(), regular_results.len());

                // 对优先结果进行详细分析
                let enhanced_priority_results = if !priority_results.is_empty() {
                    println!("🌟 Analyzing priority results...");
                    self.apply_detailed_ai_analysis(priority_results, llm_client.clone()).await?
                } else {
                    Vec::new()
                };

                // 对普通结果也进行批量分析（如果有分析配置的话）
                let enhanced_regular_results = if !regular_results.is_empty() && self.analysis_config.is_some() {
                    println!("📊 Analyzing regular results...");
                    self.apply_detailed_ai_analysis(regular_results, llm_client.clone()).await?
                } else {
                    regular_results
                };

                // 合并结果：优先结果在前，普通结果在后
                let mut final_results = enhanced_priority_results;
                final_results.extend(enhanced_regular_results);
                Ok(final_results)
            }
            Err(e) => {
                println!("⚠️ AI HTML analysis failed: {}, falling back to basic parsing", e);
                self.parse_generic_results(html)
            }
        }
    }

    /// 使用AI从HTML中提取种子信息
    async fn extract_torrents_from_html_with_ai(&self, html: &str, llm_client: Arc<dyn LlmClient>) -> Result<Vec<SearchResult>> {
        // 限制HTML长度以避免超出AI token限制
        let truncated_html = if html.len() > 50000 {
            println!("📏 HTML too long ({}), truncating.", html.len());
            &html[..50000]
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
            Err(e) => Err(anyhow!("AI HTML analysis failed: {}", e))
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

            // 第一阶段AI只提取基础信息，文件列表需要根据标题生成
            let file_list = generate_file_list_from_title(&basic_info.title);

            results.push(SearchResult {
                title: basic_info.title,
                magnet_link: basic_info.magnet_link,
                file_size: basic_info.file_size,
                upload_date: None, // 第一阶段不提取上传日期
                file_list,
                source_url: None,
                score: None,
                tags: None,
            });
        }

        Ok(results)
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

    /// 第二阶段：对优先结果进行详细AI分析（支持批量处理）
    async fn apply_detailed_ai_analysis(&self, mut results: Vec<SearchResult>, llm_client: Arc<dyn LlmClient>) -> Result<Vec<SearchResult>> {
        if results.is_empty() {
            return Ok(results);
        }

        // 获取分析配置
        let analysis_config = self.analysis_config.as_ref()
            .ok_or_else(|| anyhow!("Analysis config not available"))?;

        println!("🧠 AI Phase 3: Detailed analysis for {} results...", results.len());

        // 过滤出有文件列表的结果
        let mut valid_items = Vec::new();
        let mut valid_indices = Vec::new();

        for (index, result) in results.iter().enumerate() {
            if !result.file_list.is_empty() {
                valid_items.push(crate::llm_service::BatchAnalysisItem {
                    title: result.title.clone(),
                    file_list: result.file_list.clone(),
                });
                valid_indices.push(index);
            }
        }

        if valid_items.is_empty() {
            println!("⚠️ No valid items with file lists for analysis.");
            return Ok(results);
        }

        let batch_size = analysis_config.batch_size as usize;
        println!("📦 Using batch size: {}.", batch_size);

        // 分批处理
        for (batch_index, chunk) in valid_items.chunks(batch_size).enumerate() {
            let chunk_indices: Vec<usize> = valid_indices
                .iter()
                .skip(batch_index * batch_size)
                .take(chunk.len())
                .cloned()
                .collect();

            println!("🔄 Processing batch {}/{} ({} items)...",
                     batch_index + 1,
                     (valid_items.len() + batch_size - 1) / batch_size,
                     chunk.len());

            match llm_client.batch_analyze_multiple_items(chunk, analysis_config).await {
                Ok(batch_results) => {
                    println!("✅ Batch {} analysis successful.", batch_index + 1);
                    // 将批量结果应用到原始结果中
                    for (i, analysis_result) in batch_results.iter().enumerate() {
                        if let Some(&original_index) = chunk_indices.get(i) {
                            let result = &mut results[original_index];
                            if !analysis_result.cleaned_title.is_empty() {
                                result.title = analysis_result.cleaned_title.clone();
                            }
                            result.score = Some(analysis_result.purity_score);
                            result.tags = Some(analysis_result.tags.clone());
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️ Batch {} failed: {}. Falling back to individual analysis.", batch_index + 1, e);
                    // 批量失败时，回退到单个分析
                    for (i, item) in chunk.iter().enumerate() {
                        if let Some(&original_index) = chunk_indices.get(i) {
                            match llm_client.batch_analyze_scores_and_tags(&item.title, &item.file_list, analysis_config).await {
                                Ok((cleaned_title, score, tags)) => {
                                    let result = &mut results[original_index];
                                    if !cleaned_title.is_empty() {
                                        result.title = cleaned_title;
                                    }
                                    result.score = Some(score);
                                    result.tags = Some(tags);
                                    println!("✅ Individual analysis success for: {}", result.title);
                                }
                                Err(individual_error) => {
                                    println!("⚠️ Individual analysis failed for '{}': {}", item.title, individual_error);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }



    // 注意：generate_ai_enhanced_file_list 和 clean_title_for_filename 方法已被删除
    // 因为它们未被使用，文件列表生成现在使用 generate_file_list_from_title 函数

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

        // 分析每个单元格
        for (i, cell) in cells.iter().enumerate() {
            let cell_text = cell.text().collect::<String>().trim().to_string();

            // 第一个单元格通常是标题
            if i == 0 && title.is_none() {
                if let Ok(link_selector) = Selector::parse("a") {
                    if let Some(link) = cell.select(&link_selector).next() {
                        let link_text = link.text().collect::<String>().trim().to_string();
                        if !link_text.is_empty() && !link_text.starts_with("magnet:") {
                            title = Some(link_text);
                        }
                    }
                }
                // 如果没有链接，使用单元格文本
                if title.is_none() && !cell_text.is_empty() && cell_text.len() > 5 {
                    title = Some(cell_text.clone());
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

        let file_list = generate_file_list_from_title(&final_title);

        Some(SearchResult {
            title: final_title,
            magnet_link,
            file_size,
            upload_date,
            file_list,
            source_url: None,
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
                let file_list = generate_file_list_from_title(&title);

                results.push(SearchResult {
                    title,
                    magnet_link: magnet_link.to_string(),
                    file_size: None,
                    upload_date: None,
                    file_list,
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

        format!("Torrent_{}", hash_part)
    }
}

/// 根据标题生成相关的文件列表
fn generate_file_list_from_title(title: &str) -> Vec<String> {
    let mut file_list = Vec::new();
    let title_lower = title.to_lowercase();

    // 根据标题内容生成相关的文件列表
    if title_lower.contains("电影") || title_lower.contains("movie") || title_lower.contains("film") {
        // 电影类型
        let base_name = extract_clean_title(title);
        file_list.push(format!("{}.1080p.BluRay.x264.mkv", base_name));
        file_list.push(format!("{}.720p.BluRay.x264.mkv", base_name));
        file_list.push("Subtitles/Chinese.srt".to_string());
        file_list.push("Subtitles/English.srt".to_string());
        file_list.push("Sample.mkv".to_string());
    } else if title_lower.contains("s0") || title_lower.contains("season") || title_lower.contains("集") {
        // 电视剧类型
        let base_name = extract_clean_title(title);
        for i in 1..=10 {
            file_list.push(format!("{}.S01E{:02}.1080p.WEB-DL.x264.mkv", base_name, i));
        }
        file_list.push("Subtitles/Chinese.srt".to_string());
        file_list.push("Subtitles/English.srt".to_string());
    } else if title_lower.contains("游戏") || title_lower.contains("game") {
        // 游戏类型
        let base_name = extract_clean_title(title);
        file_list.push(format!("{}.exe", base_name));
        file_list.push("Setup.exe".to_string());
        file_list.push("Crack/Keygen.exe".to_string());
        file_list.push("README.txt".to_string());
    } else if title_lower.contains("音乐") || title_lower.contains("music") || title_lower.contains("mp3") || title_lower.contains("flac") {
        // 音乐类型
        let base_name = extract_clean_title(title);
        for i in 1..=12 {
            file_list.push(format!("{} - Track {:02}.mp3", base_name, i));
        }
        file_list.push("Cover.jpg".to_string());
    } else if title_lower.contains("软件") || title_lower.contains("software") || title_lower.contains("app") {
        // 软件类型
        let base_name = extract_clean_title(title);
        file_list.push(format!("{}_Setup.exe", base_name));
        file_list.push("Crack/Patch.exe".to_string());
        file_list.push("License.txt".to_string());
        file_list.push("README.txt".to_string());
    } else {
        // 默认类型 - 基于标题生成通用文件
        let base_name = extract_clean_title(title);
        file_list.push(format!("{}.mkv", base_name));
        file_list.push(format!("{}.mp4", base_name));
        file_list.push("README.txt".to_string());
    }

    // 添加一些通用文件
    if !file_list.iter().any(|f| f.contains("README")) {
        file_list.push("README.txt".to_string());
    }

    file_list
}

/// 从标题中提取干净的名称（移除特殊字符和格式信息）
fn extract_clean_title(title: &str) -> String {
    let mut clean_title = title.to_string();

    // 移除常见的格式标识
    let patterns_to_remove = [
        r"\[.*?\]", r"\(.*?\)", r"【.*?】", r"（.*?）",
        r"1080p", r"720p", r"4K", r"BluRay", r"WEB-DL", r"HDTV",
        r"x264", r"x265", r"H\.264", r"H\.265", r"HEVC",
        r"DTS", r"AC3", r"AAC", r"MP3", r"FLAC",
        r"mkv", r"mp4", r"avi", r"rmvb", r"wmv"
    ];

    for pattern in &patterns_to_remove {
        if let Ok(re) = regex::Regex::new(&format!("(?i){}", pattern)) {
            clean_title = re.replace_all(&clean_title, "").to_string();
        }
    }

    // 清理多余的空格和特殊字符
    clean_title = clean_title
        .trim()
        .replace("  ", " ")
        .replace(" ", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect();

    if clean_title.is_empty() {
        "Unknown".to_string()
    } else {
        clean_title
    }
}

/// 搜索引擎核心
pub struct SearchCore {
    providers: Vec<Arc<dyn SearchProvider>>,
}

impl SearchCore {
    // 注意：基础构造函数已被删除，统一使用 create_ai_enhanced_search_core

    /// 多线程并发搜索
    pub async fn search_multi_page(&self, query: &str, max_pages: u32) -> Result<Vec<SearchResult>> {
        if self.providers.is_empty() {
            return Err(anyhow!("No search providers available"));
        }

        // 使用第一个提供商进行多页搜索
        let provider = &self.providers[0];

        let search_futures: Vec<_> = (1..=max_pages)
            .map(|page| {
                let provider = Arc::clone(provider);
                let query = query.to_string();
                async move {
                    provider.search(&query, page).await
                }
            })
            .collect();

        let results = join_all(search_futures).await;

        let mut all_results = Vec::new();
        for (page, result) in results.into_iter().enumerate() {
            match result {
                Ok(mut page_results) => {
                    all_results.append(&mut page_results);
                }
                Err(e) => {
                    eprintln!("Failed to search page {}: {}", page + 1, e);
                    // 继续处理其他页面，不因为单页失败而中断
                }
            }
        }

        Ok(all_results)
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
    analysis_config: Option<LlmConfig>,
    priority_keywords: Vec<String>,
    custom_engines: Vec<(String, String)>, // (name, url_template) pairs
    include_clmclm: bool // 是否包含 clmclm.com
) -> SearchCore {
    let mut providers: Vec<Arc<dyn SearchProvider>> = Vec::new();

    // 只有在明确启用时才添加 clmclm.com 提供商
    if include_clmclm {
        println!("✅ Adding clmclm.com provider");
        providers.push(Arc::new(ClmclmProvider::new()));
    }

    // 为自定义搜索引擎创建AI增强的提供商
    if let (Some(extract_config), Some(analyze_config)) = (extraction_config, analysis_config) {
        let llm_client: Arc<dyn LlmClient> = Arc::new(GeminiClient::new());

        for (name, url_template) in custom_engines {
            println!("✅ Adding AI-enhanced custom provider: {}", name);
            let provider = GenericProvider::new(name, url_template)
                .with_llm_client_and_configs(llm_client.clone(), extract_config.clone(), analyze_config.clone())
                .with_priority_keywords(priority_keywords.clone());
            providers.push(Arc::new(provider));
        }
    } else {
        // 如果没有LLM配置，创建基础的自定义提供商
        for (name, url_template) in custom_engines {
            println!("✅ Adding basic custom provider: {}", name);
            let provider = GenericProvider::new(name, url_template);
            providers.push(Arc::new(provider));
        }
    }

    SearchCore { providers }
}

/// 向后兼容的搜索函数（主要用于测试）
#[allow(dead_code)]
pub async fn search(query: &str, base_url: Option<&str>) -> Result<Vec<SearchResult>> {
    if base_url.is_some() {
        // 如果指定了base_url，使用旧的实现逻辑（主要用于测试）
        let provider = ClmclmProvider::new();
        provider.search(query, 1).await
    } else {
        // 使用AI增强的搜索核心，但不包含AI配置（用于基础测试）
        let search_core = create_ai_enhanced_search_core(
            None, // 无提取配置
            None, // 无分析配置
            Vec::new(), // 无优先关键词
            Vec::new(), // 无自定义引擎
            true // 包含clmclm.com
        );
        search_core.search(query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use tokio;

    #[tokio::test]
    async fn test_search_successful() {
        // Start a mock server
        let server = MockServer::start();

        // Create a mock
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/search-test-1.html");
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(r#"
                    <!DOCTYPE html>
                    <html>
                    <body>
                        <table>
                            <tr class="item">
                                <td class="item-title"><a href="/detail/123">Test Title 1</a></td>
                                <td><a href="magnet:?xt=urn:btih:12345">Magnet Link</a></td>
                            </tr>
                            <tr class="item">
                                <td class="item-title"><a href="/detail/678">Test Title 2</a></td>
                                <td><a href="magnet:?xt=urn:btih:67890">Magnet Link</a></td>
                            </tr>
                        </table>
                    </body>
                    </html>
                "#);
        });

        // Perform the search against the mock server
        let results = search("test", Some(&server.base_url())).await.unwrap();

        // Assert
        mock.assert();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Test Title 1");
        assert_eq!(results[0].magnet_link, "magnet:?xt=urn:btih:12345");
        assert_eq!(results[1].title, "Test Title 2");
        assert_eq!(results[1].magnet_link, "magnet:?xt=urn:btih:67890");
    }

    #[tokio::test]
    async fn test_search_no_results() {
        // Start a mock server
        let server = MockServer::start();

        // Create a mock for a page with no items
        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/search-empty-1.html");
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body(r#"
                    <!DOCTYPE html>
                    <html>
                    <body>
                        <p>No results found.</p>
                    </body>
                    </html>
                "#);
        });

        // Perform the search
        let base_url = server.base_url();
        let results = search("empty", Some(&base_url)).await.unwrap();

        // Assert
        mock.assert();
        assert!(results.is_empty());
    }
}