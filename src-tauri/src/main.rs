#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 引入我们的新模块
mod llm_service;
use crate::llm_service::LlmClient;
// 引入需要的模块
mod searcher;
mod app_state;
mod filter;

use tauri::Manager;
use regex::Regex;

// ============ AI分析命令 ============

/// 使用正则表达式作为后备方案清理标题
fn clean_title_fallback(title: &str) -> String {
    // 移除常见的广告标记，如 [y5y4.com] 或 【...】
    let re_brackets = Regex::new(r"\[.*?\]|【.*?】").unwrap();
    let title = re_brackets.replace_all(title, "");

    // 移除常见的URL和推广信息
    let re_urls = Regex::new(r"(?i)(www\.\S+\.\S+|https?://\S+)").unwrap();
    let title = re_urls.replace_all(&title, "");

    // 清理多余的空格
    title.trim().replace("  ", " ")
}


#[tauri::command]
async fn analyze_resource(
    result: searcher::SearchResult,
    llm_config: llm_service::LlmConfig,
) -> Result<llm_service::DetailedAnalysisResult, String> {
    let client = llm_service::GeminiClient::new();

    match client.batch_analyze_scores_and_tags(&result.title, &result.file_list, &llm_config).await {
        Ok((cleaned_title, score, tags)) => {
            // --- 调试输出 ---
            println!("[AI DEBUG] Original Title: '{}'", result.title);
            println!("[AI DEBUG] Cleaned Title: '{}'", cleaned_title);
            // --- 调试输出结束 ---

            let final_title = if cleaned_title.is_empty() {
                clean_title_fallback(&result.title)
            } else {
                cleaned_title
            };

            Ok(llm_service::DetailedAnalysisResult {
                title: final_title,
                purity_score: score,
                tags,
                magnet_link: result.magnet_link,
                file_size: result.file_size,
                file_list: result.file_list,
            })
        }
        Err(e) => Err(e.to_string()),
    }
}


// ============ 收藏夹相关命令 ============

#[tauri::command]
async fn add_to_favorites(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    title: String,
    magnet_link: String,
    file_size: Option<String>,
    file_list: Vec<String>,
) -> Result<app_state::FavoriteItem, String> {
    let result = app_state::add_to_favorites(&state, title, magnet_link, file_size, file_list)
        .map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
async fn get_all_favorites(state: tauri::State<'_, app_state::AppState>) -> Result<Vec<app_state::FavoriteItem>, String> {
    Ok(app_state::get_all_favorites(&state))
}

#[tauri::command]
async fn remove_from_favorites(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    id: String,
) -> Result<(), String> {
    app_state::remove_from_favorites(&state, id).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn search_favorites(
    state: tauri::State<'_, app_state::AppState>,
    query: String,
) -> Result<Vec<app_state::FavoriteItem>, String> {
    Ok(app_state::search_favorites(&state, query))
}



#[tauri::command]
async fn search_multi_page(
    state: tauri::State<'_, app_state::AppState>,
    keyword: String,
    max_pages: Option<u32>,
) -> Result<Vec<searcher::SearchResult>, String> {
    let pages = max_pages.unwrap_or(3);

    // 获取启用的搜索引擎
    let engines = app_state::get_all_engines(&state);
    let enabled_engines: Vec<_> = engines.into_iter().filter(|e| e.is_enabled).collect();

    if enabled_engines.is_empty() {
        return Err("No enabled search engines found. Please enable at least one search engine in Settings.".to_string());
    }

    // 获取优先关键词
    let priority_keywords = app_state::get_all_priority_keywords(&state);
    let priority_keyword_strings: Vec<String> = priority_keywords.iter()
        .map(|pk| pk.keyword.clone())
        .collect();

    // 获取LLM配置
    let llm_config = app_state::get_llm_config(&state);

    // 检查是否有有效的API密钥
    let has_extraction_config = !llm_config.extraction_config.api_key.is_empty();
    let has_analysis_config = !llm_config.analysis_config.api_key.is_empty();

    println!("🔧 LLM extraction config available: {}", has_extraction_config);
    println!("🔧 LLM analysis config available: {}", has_analysis_config);

    // 分离 clmclm.com 和自定义搜索引擎
    let clmclm_enabled = enabled_engines.iter().any(|e| &e.name == "clmclm.com");
    let custom_engines: Vec<_> = enabled_engines.iter()
        .filter(|e| &e.name != "clmclm.com")
        .map(|e| (e.name.clone(), e.url_template.clone()))
        .collect();

    // 转换为llm_service::LlmConfig格式
    let extraction_config = if has_extraction_config {
        Some(llm_service::LlmConfig {
            provider: llm_config.extraction_config.provider.clone(),
            api_key: llm_config.extraction_config.api_key.clone(),
            api_base: llm_config.extraction_config.api_base.clone(),
            model: llm_config.extraction_config.model.clone(),
        })
    } else {
        None
    };

    let analysis_config = if has_analysis_config {
        Some(llm_service::LlmConfig {
            provider: llm_config.analysis_config.provider.clone(),
            api_key: llm_config.analysis_config.api_key.clone(),
            api_base: llm_config.analysis_config.api_base.clone(),
            model: llm_config.analysis_config.model.clone(),
        })
    } else {
        None
    };

    // 创建搜索核心，只包含启用的搜索引擎
    let search_core = if !custom_engines.is_empty() || clmclm_enabled {
        println!("🔧 Creating search core with {} custom engines, clmclm.com: {}",
                custom_engines.len(), clmclm_enabled);
        searcher::create_ai_enhanced_search_core(
            extraction_config,
            analysis_config,
            priority_keyword_strings,
            custom_engines,
            clmclm_enabled
        )
    } else {
        return Err("No enabled search engines found. Please enable at least one search engine.".to_string());
    };

    search_core.search_multi_page(keyword.as_str(), pages).await.map_err(|e| e.to_string())
}

// ============ 搜索引擎相关命令 ============

#[tauri::command]
async fn add_search_engine(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    name: String,
    url_template: String,
) -> Result<app_state::SearchEngine, String> {
    let result = app_state::add_search_engine(&state, name, url_template)
        .map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
async fn get_all_engines(state: tauri::State<'_, app_state::AppState>) -> Result<Vec<app_state::SearchEngine>, String> {
    Ok(app_state::get_all_engines(&state))
}

#[tauri::command]
async fn update_engine_status(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    id: String,
    is_enabled: bool,
) -> Result<(), String> {
    app_state::update_engine_status(&state, id, is_enabled).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn delete_engine(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    id: String,
) -> Result<(), String> {
    app_state::delete_engine(&state, id).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(())
}

// ============ 优先关键词相关命令 ============

#[tauri::command]
async fn add_priority_keyword(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    keyword: String,
) -> Result<app_state::PriorityKeyword, String> {
    let result = app_state::add_priority_keyword(&state, keyword)
        .map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
async fn get_all_priority_keywords(state: tauri::State<'_, app_state::AppState>) -> Result<Vec<app_state::PriorityKeyword>, String> {
    Ok(app_state::get_all_priority_keywords(&state))
}

#[tauri::command]
async fn delete_priority_keyword(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    id: String,
) -> Result<(), String> {
    app_state::delete_priority_keyword(&state, id).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn test_connection(config: llm_service::LlmConfig) -> Result<String, String> {
    llm_service::test_connection(&config).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn test_extraction_connection(config: app_state::SingleLlmConfig) -> Result<String, String> {
    let llm_config = llm_service::LlmConfig {
        provider: config.provider,
        api_key: config.api_key,
        api_base: config.api_base,
        model: config.model,
    };
    llm_service::test_connection(&llm_config).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn test_analysis_connection(config: app_state::SingleLlmConfig) -> Result<String, String> {
    let llm_config = llm_service::LlmConfig {
        provider: config.provider,
        api_key: config.api_key,
        api_base: config.api_base,
        model: config.model,
    };
    llm_service::test_connection(&llm_config).await.map_err(|e| e.to_string())
}

// 注意：load_llm_config_from_app 和 load_llm_config_from_file 函数已被删除
// 因为它们未被使用，LLM配置现在通过前端直接传递

// ============ LLM 配置相关命令 ============

#[tauri::command]
async fn get_llm_config(state: tauri::State<'_, app_state::AppState>) -> Result<app_state::LlmConfig, String> {
    Ok(app_state::get_llm_config(&state))
}

#[tauri::command]
async fn update_llm_config(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    config: app_state::LlmConfig,
) -> Result<(), String> {
    app_state::update_llm_config(&state, config).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(())
}

// ============ 搜索设置相关命令 ============

#[tauri::command]
async fn get_search_settings(state: tauri::State<'_, app_state::AppState>) -> Result<app_state::SearchSettings, String> {
    Ok(app_state::get_search_settings(&state))
}

#[tauri::command]
async fn update_search_settings(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    settings: app_state::SearchSettings,
) -> Result<(), String> {
    app_state::update_search_settings(&state, settings).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 初始化应用状态
            let app_state = app_state::init_app_state(app.handle())
                .expect("Failed to initialize app state");
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search_multi_page,
            test_connection,
            test_extraction_connection,
            test_analysis_connection,
            analyze_resource,
            // 收藏夹命令
            add_to_favorites,
            get_all_favorites,
            remove_from_favorites,
            search_favorites,
            // 搜索引擎命令
            add_search_engine,
            get_all_engines,
            update_engine_status,
            delete_engine,
            // 优先关键词命令
            add_priority_keyword,
            get_all_priority_keywords,
            delete_priority_keyword,
            // LLM 配置命令
            get_llm_config,
            update_llm_config,
            // 搜索设置命令
            get_search_settings,
            update_search_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
