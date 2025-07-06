use crate::searcher::SearchResult;
use crate::llm_service::LlmClient;
use std::sync::Arc;
use anyhow::Result;
use futures::future::join_all;

const AD_SCORE_THRESHOLD: f32 = 0.5;
const MAX_CONCURRENT_EVALUATIONS: usize = 10;

/// 双轨筛选引擎：优先级筛选 + LLM智能筛选
pub async fn filter_results(
    results: &[SearchResult],
    llm_client: Arc<dyn LlmClient>,
) -> Result<Vec<SearchResult>> {
    if results.is_empty() {
        return Ok(Vec::new());
    }

    // 第一轨：优先级筛选 - 检查已知的高质量来源标记
    let priority_results = apply_priority_filter(results);
    if !priority_results.is_empty() {
        println!("Found {} priority results, skipping LLM evaluation", priority_results.len());
        return Ok(priority_results);
    }

    // 第二轨：LLM智能筛选
    println!("Applying LLM-based filtering to {} results", results.len());
    apply_llm_filter(results, llm_client).await
}

/// 优先级筛选：基于已知的高质量来源标记
fn apply_priority_filter(results: &[SearchResult]) -> Vec<SearchResult> {
    let priority_markers = [
        "***REMOVED***.com@",
        "高清电影",
        "蓝光原盘",
        "4K",
        "1080p",
    ];

    results
        .iter()
        .filter(|result| {
            priority_markers.iter().any(|marker| result.title.contains(marker))
        })
        .cloned()
        .collect()
}

/// LLM智能筛选：使用AI评估广告可能性
async fn apply_llm_filter(
    results: &[SearchResult],
    llm_client: Arc<dyn LlmClient>,
) -> Result<Vec<SearchResult>> {
    // 分批处理以避免过多并发请求
    let chunks: Vec<_> = results.chunks(MAX_CONCURRENT_EVALUATIONS).collect();
    let mut filtered_results = Vec::new();

    for chunk in chunks {
        let evaluation_futures: Vec<_> = chunk
            .iter()
            .map(|result| {
                let client = Arc::clone(&llm_client);
                let title = result.title.clone();
                async move {
                    (result, client.evaluate_ad(&title).await)
                }
            })
            .collect();

        let evaluation_results = join_all(evaluation_futures).await;

        for (result, eval_res) in evaluation_results {
            match eval_res {
                Ok(score) => {
                    if score < AD_SCORE_THRESHOLD {
                        filtered_results.push(result.clone());
                        println!("✓ Kept: {} (score: {:.2})", result.title, score);
                    } else {
                        println!("✗ Filtered: {} (score: {:.2})", result.title, score);
                    }
                }
                Err(e) => {
                    // 如果LLM评估失败，默认保留结果
                    println!("⚠ LLM evaluation failed for '{}': {}, keeping result", result.title, e);
                    filtered_results.push(result.clone());
                }
            }
        }
    }

    Ok(filtered_results)
}

/// 结果富化：为筛选后的结果添加智能标签
pub async fn enrich_results(
    results: &[SearchResult],
    llm_client: Arc<dyn LlmClient>,
) -> Result<Vec<SearchResult>> {
    if results.is_empty() {
        return Ok(Vec::new());
    }

    println!("Enriching {} results with smart tags", results.len());

    let enrichment_futures: Vec<_> = results
        .iter()
        .map(|result| {
            let client = Arc::clone(&llm_client);
            let title = result.title.clone();
            async move {
                (result, client.enrich_result(&title).await)
            }
        })
        .collect();

    let enrichment_results = join_all(enrichment_futures).await;
    let mut enriched_results = Vec::new();

    for (result, enrich_res) in enrichment_results {
        let enriched_result = result.clone();

        match enrich_res {
            Ok(tags) => {
                // 这里可以将标签信息添加到结果中
                // 目前SearchResult结构还没有tags字段，所以先记录日志
                if !tags.is_empty() {
                    println!("📝 Tags for '{}': {:?}", result.title, tags);
                }
            }
            Err(e) => {
                println!("⚠ Enrichment failed for '{}': {}", result.title, e);
            }
        }

        enriched_results.push(enriched_result);
    }

    Ok(enriched_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::searcher::SearchResult;
    use async_trait::async_trait;
    use anyhow::Result;

    struct MockLlmClient;

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn evaluate_ad(&self, title: &str) -> Result<f32> {
            if title.contains("ad") {
                Ok(0.8)
            } else {
                Ok(0.2)
            }
        }

        async fn enrich_result(&self, _title: &str) -> Result<Vec<String>> {
            Ok(vec!["test".to_string()])
        }
    }

    #[tokio::test]
    async fn test_filter_results_with_priority_marker() {
        let results = vec![
            SearchResult {
                title: "Result 1".to_string(),
                magnet_link: "magnet:1".to_string(),
                file_size: None,
                upload_date: None,
            },
            SearchResult {
                title: "***REMOVED***.com@ Result 2".to_string(),
                magnet_link: "magnet:2".to_string(),
                file_size: None,
                upload_date: None,
            },
        ];
        let client = Arc::new(MockLlmClient);
        let filtered = filter_results(&results, client).await.unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "***REMOVED***.com@ Result 2");
    }

    #[tokio::test]
    async fn test_filter_results_with_llm() {
        let results = vec![
            SearchResult {
                title: "This is a clean result".to_string(),
                magnet_link: "magnet:1".to_string(),
                file_size: None,
                upload_date: None,
            },
            SearchResult {
                title: "This is an ad result".to_string(),
                magnet_link: "magnet:2".to_string(),
                file_size: None,
                upload_date: None,
            },
        ];
        let client = Arc::new(MockLlmClient);
        let filtered = filter_results(&results, client).await.unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "This is a clean result");
    }
}