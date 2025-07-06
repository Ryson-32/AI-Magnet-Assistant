use anyhow::Result;
use reqwest;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct SearchResult {
    pub title: String,
    pub magnet_link: String,
}

pub async fn search(query: &str, base_url: Option<&str>) -> Result<Vec<SearchResult>> {
    let url = match base_url {
        Some(base) => format!("{}/search-{}-1-1-1.html", base, query),
        None => format!("http://clmclm.com/search-{}-1-1-1.html", query),
    };
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .build()?;
    let response = client.get(&url).send().await?.text().await?;
    let document = Html::parse_document(&response);

    let row_selector = Selector::parse("div.ssbox").unwrap();
    let title_selector = Selector::parse("div.title > h3 > a").unwrap();
    let magnet_selector = Selector::parse("div.sbar a[href^=\"magnet:\"]").unwrap();

    let mut results = Vec::new();

    for element in document.select(&row_selector) {
        let title_element = element.select(&title_selector).next();
        let magnet_element = element.select(&magnet_selector).next();

        if let (Some(title_node), Some(magnet_node)) = (title_element, magnet_element) {
            let title = title_node.text().collect::<String>().trim().to_string();
            if let Some(magnet_link) = magnet_node.value().attr("href") {
                results.push(SearchResult {
                    title,
                    magnet_link: magnet_link.to_string(),
                });
            }
        }
    }

    Ok(results)
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