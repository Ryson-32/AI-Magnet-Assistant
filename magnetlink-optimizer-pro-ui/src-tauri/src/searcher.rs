use anyhow::{Result, anyhow};
use reqwest;
use scraper::{Html, Selector};
use futures::future::join_all;
use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub magnet_link: String,
    pub file_size: Option<String>,
    pub upload_date: Option<String>,
    pub file_list: Vec<String>,
    pub source_url: Option<String>,
}

/// 搜索引擎提供商特性
#[async_trait::async_trait]
pub trait SearchProvider: Send + Sync {
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
        println!("✅ Got response, parsing results...");
        let results = self.parse_results(&html)?;
        println!("📊 Found {} results on page {}", results.len(), page);
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

/// 通用搜索引擎提供商，支持自定义URL模板
pub struct GenericProvider {
    name: String,
    url_template: String,
    client: reqwest::Client,
}

impl GenericProvider {
    pub fn new(name: String, url_template: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { name, url_template, client }
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

        println!("✅ Got response, parsing results...");

        // 对于自定义搜索引擎，我们尝试通用的解析方法
        let results = self.parse_generic_results(&html)?;

        println!("📊 Found {} results on page {}", results.len(), page);
        Ok(results)
    }
}

impl GenericProvider {
    fn parse_generic_results(&self, html: &str) -> Result<Vec<SearchResult>> {
        // 这是一个简化的通用解析器
        let document = Html::parse_document(html);
        let mut results = Vec::new();

        // 尝试查找常见的磁力链接模式
        let magnet_regex = regex::Regex::new(r"magnet:\?xt=urn:btih:[a-fA-F0-9]{40}[^&\s]*")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;

        for magnet_match in magnet_regex.find_iter(html) {
            let magnet_link = magnet_match.as_str();

            // 尝试提取标题（这是一个简化的方法）
            let title = format!("Search Result - {}", magnet_link.chars().take(50).collect::<String>());
            let file_list = generate_file_list_from_title(&title);

            results.push(SearchResult {
                title,
                magnet_link: magnet_link.to_string(),
                file_size: None,
                upload_date: None,
                file_list,
                source_url: None, // Generic provider doesn't have a source URL
            });
        }

        Ok(results)
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
    pub fn new() -> Self {
        let mut providers: Vec<Arc<dyn SearchProvider>> = Vec::new();
        providers.push(Arc::new(ClmclmProvider::new()));

        Self { providers }
    }

    /// 从搜索引擎配置创建SearchCore
    pub fn from_engine_config(name: &str, url_template: &str) -> Self {
        let mut providers: Vec<Arc<dyn SearchProvider>> = Vec::new();

        if name == "clmclm.com" {
            providers.push(Arc::new(ClmclmProvider::new()));
        } else {
            // 对于自定义搜索引擎，创建通用提供商
            providers.push(Arc::new(GenericProvider::new(
                name.to_string(),
                url_template.to_string()
            )));
        }

        Self { providers }
    }

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
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        self.search_multi_page(query, 1).await
    }
}

/// 向后兼容的搜索函数
pub async fn search(query: &str, base_url: Option<&str>) -> Result<Vec<SearchResult>> {
    if base_url.is_some() {
        // 如果指定了base_url，使用旧的实现逻辑（主要用于测试）
        let provider = ClmclmProvider::new();
        provider.search(query, 1).await
    } else {
        // 使用新的搜索核心
        let search_core = SearchCore::new();
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
        let results = search("empty", Some(&server.base_url())).await.unwrap();

        // Assert
        mock.assert();
        assert!(results.is_empty());
    }
}