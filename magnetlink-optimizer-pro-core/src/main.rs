mod filter;
mod searcher;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: magnetlink-optimizer-pro-core <search-keyword>");
        return Ok(());
    }
    let keyword = &args[1];

    println!("Searching for: {}", keyword);

    let search_results = searcher::search(keyword, None).await?;
    let filtered_results = filter::filter_results(&search_results);

    if filtered_results.is_empty() {
        println!("No results found.");
    } else {
        println!("Found {} results:", filtered_results.len());
        for result in filtered_results {
            println!("- Title: {}", result.title);
            println!("  Magnet: {}", result.magnet_link);
        }
    }

    Ok(())
}