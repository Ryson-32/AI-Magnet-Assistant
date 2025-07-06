#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 引入我们的新模块
mod llm_service;
// 引入需要的模块
mod searcher;

// 使用Tauri的 `command` 宏
#[tauri::command]
async fn analyze_resource(title: String, file_list: Vec<String>, config: llm_service::LlmConfig) -> Result<llm_service::AnalysisResult, String> {
    llm_service::analyze_resource_with_gemini(title, file_list, &config).await
}



#[tauri::command]
async fn search_multi_page(keyword: String, max_pages: Option<u32>) -> Result<Vec<searcher::SearchResult>, String> {
    let pages = max_pages.unwrap_or(3);
    let search_core = searcher::SearchCore::new();
    search_core.search_multi_page(&keyword, pages).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn test_connection(config: llm_service::LlmConfig) -> Result<String, String> {
    llm_service::test_connection(&config).await
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            analyze_resource,
            search_multi_page,
            test_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
