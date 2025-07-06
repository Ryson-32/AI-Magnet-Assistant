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
        let size_selector = Selector::parse("div.sbar span.size")
            .map_err(|e| anyhow!("Invalid CSS selector: {}", e))?;

        let mut results = Vec::new();

        for element in document.select(&row_selector) {
            let title_element = element.select(&title_selector).next();
            let magnet_element = element.select(&magnet_selector).next();
            let size_element = element.select(&size_selector).next();

            if let (Some(title_node), Some(magnet_node)) = (title_element, magnet_element) {
                let title = title_node.text().collect::<String>().trim().to_string();
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

                    results.push(SearchResult {
                        title,
                        magnet_link: magnet_link.to_string(),
                        file_size,
                        upload_date: None, // clmclm.com doesn't provide upload date
                    });
                }
            }
        }

        Ok(results)
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