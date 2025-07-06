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
    let client = llm_service::GeminiClient::new(llm_config);

    match client.batch_analyze_scores_and_tags(&result.title, &result.file_list).await {
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
    llm_config: Option<llm_service::LlmConfig>
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

    // 使用前端传递的LLM配置（如果有的话）
    println!("🔧 LLM config received from frontend: {}", llm_config.is_some());

    // 分离 clmclm.com 和自定义搜索引擎
    let clmclm_enabled = enabled_engines.iter().any(|e| e.name == "clmclm.com");
    let custom_engines: Vec<_> = enabled_engines.iter()
        .filter(|e| e.name != "clmclm.com")
        .map(|e| (e.name.clone(), e.url_template.clone()))
        .collect();

    // 创建搜索核心，只包含启用的搜索引擎
    let search_core = if !custom_engines.is_empty() || clmclm_enabled {
        println!("🔧 Creating search core with {} custom engines, clmclm.com: {}",
                custom_engines.len(), clmclm_enabled);
        searcher::create_ai_enhanced_search_core(
            llm_config,
            priority_keyword_strings,
            custom_engines,
            clmclm_enabled
        )
    } else {
        return Err("No enabled search engines found. Please enable at least one search engine.".to_string());
    };

    search_core.search_multi_page(&keyword, pages).await.map_err(|e| e.to_string())
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

/// 从应用中加载LLM配置的辅助函数
async fn load_llm_config_from_app(app_handle: &tauri::AppHandle) -> Option<llm_service::LlmConfig> {
    // 尝试从Tauri store加载LLM配置（与前端保持一致）
    let app_data_dir = app_handle.path().app_data_dir().ok()?;

    // Tauri store插件将文件保存在 app_data_dir/stores/ 目录下
    let store_path = app_data_dir.join("stores").join("settings.json");

    println!("🔍 Looking for LLM config at: {:?}", store_path);

    if !store_path.exists() {
        // 尝试旧的路径作为备用
        let fallback_path = app_data_dir.join("settings.json");
        println!("🔍 Trying fallback path: {:?}", fallback_path);

        if !fallback_path.exists() {
            println!("⚠️ LLM config not found at either location, AI features will be disabled");
            return None;
        } else {
            return load_llm_config_from_file(&fallback_path);
        }
    }

    load_llm_config_from_file(&store_path)
}

/// 从指定文件加载LLM配置
fn load_llm_config_from_file(file_path: &std::path::Path) -> Option<llm_service::LlmConfig> {
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            println!("📄 Store file content length: {} bytes", content.len());

            // 解析整个store文件
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(store_data) => {
                    println!("📊 Store data keys: {:?}", store_data.as_object().map(|obj| obj.keys().collect::<Vec<_>>()));

                    // 从store中提取llm_config
                    if let Some(llm_config_value) = store_data.get("llm_config") {
                        println!("🔧 Found llm_config in store");
                        match serde_json::from_value::<llm_service::LlmConfig>(llm_config_value.clone()) {
                            Ok(config) => {
                                // 验证配置是否完整
                                if config.api_key.trim().is_empty() {
                                    println!("⚠️ LLM config found but API key is empty, AI features will be disabled");
                                    return None;
                                }
                                println!("✅ LLM config loaded successfully from store (provider: {}, model: {})",
                                        config.provider, config.model);
                                Some(config)
                            }
                            Err(e) => {
                                println!("❌ Failed to parse LLM config from store: {}", e);
                                None
                            }
                        }
                    } else {
                        println!("⚠️ LLM config key not found in store, AI features will be disabled");
                        None
                    }
                }
                Err(e) => {
                    println!("❌ Failed to parse store file as JSON: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to read store file: {}", e);
            None
        }
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
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
            delete_priority_keyword
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
