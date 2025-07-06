use crate::searcher::SearchResult;

pub fn filter_results(results: &[SearchResult]) -> Vec<&SearchResult> {
    let priority_results: Vec<&SearchResult> = results
        .iter()
        .filter(|r| r.title.contains("***REMOVED***.com@"))
        .collect();

    if !priority_results.is_empty() {
        priority_results
    } else {
        results.iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::searcher::SearchResult;

    #[test]
    fn test_filter_results_with_priority_marker() {
        let results = vec![
            SearchResult {
                title: "Result 1".to_string(),
                magnet_link: "magnet:1".to_string(),
            },
            SearchResult {
                title: "***REMOVED***.com@ Result 2".to_string(),
                magnet_link: "magnet:2".to_string(),
            },
            SearchResult {
                title: "Result 3".to_string(),
                magnet_link: "magnet:3".to_string(),
            },
            SearchResult {
                title: "Another ***REMOVED***.com@ Result 4".to_string(),
                magnet_link: "magnet:4".to_string(),
            },
        ];

        let filtered = filter_results(&results);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].title, "***REMOVED***.com@ Result 2");
        assert_eq!(filtered[1].title, "Another ***REMOVED***.com@ Result 4");
    }

    #[test]
    fn test_filter_results_without_priority_marker() {
        let results = vec![
            SearchResult {
                title: "Result 1".to_string(),
                magnet_link: "magnet:1".to_string(),
            },
            SearchResult {
                title: "Result 2".to_string(),
                magnet_link: "magnet:2".to_string(),
            },
        ];

        let filtered = filter_results(&results);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].title, "Result 1");
        assert_eq!(filtered[1].title, "Result 2");
    }

    #[test]
    fn test_filter_results_empty_input() {
        let results: Vec<SearchResult> = vec![];
        let filtered = filter_results(&results);
        assert!(filtered.is_empty());
    }
}