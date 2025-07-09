#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 引入我们的新模块
mod llm_service;
use crate::llm_service::LlmClient;
// 引入需要的模块
mod searcher;
mod app_state;

use tauri::Manager;
use regex::Regex;
use searcher::SearchCore;

// ============ 辅助函数 ============

/// 从 AppState 构建 LLM 配置
fn build_llm_configs(app_state: &app_state::AppState) -> (Option<llm_service::LlmConfig>, Option<llm_service::LlmConfig>) {
    let llm_config = app_state::get_llm_config(app_state);

    let extraction_config = if !llm_config.extraction_config.api_key.is_empty() {
        Some(llm_service::LlmConfig {
            provider: llm_config.extraction_config.provider.clone(),
            api_key: llm_config.extraction_config.api_key.clone(),
            api_base: llm_config.extraction_config.api_base.clone(),
            model: llm_config.extraction_config.model.clone(),
            batch_size: llm_config.extraction_config.batch_size,
        })
    } else {
        None
    };

    let analysis_config = if !llm_config.analysis_config.api_key.is_empty() {
        Some(llm_service::LlmConfig {
            provider: llm_config.analysis_config.provider.clone(),
            api_key: llm_config.analysis_config.api_key.clone(),
            api_base: llm_config.analysis_config.api_base.clone(),
            model: llm_config.analysis_config.model.clone(),
            batch_size: llm_config.analysis_config.batch_size,
        })
    } else {
        None
    };

    (extraction_config, analysis_config)
}

/// 从 AppState 获取启用的搜索引擎
fn get_active_engines(app_state: &app_state::AppState) -> Vec<app_state::SearchEngine> {
    app_state::get_all_engines(app_state)
        .into_iter()
        .filter(|e| e.is_enabled)
        .collect()
}

/// 从 AppState 获取优先关键词
fn get_priority_keywords(app_state: &app_state::AppState) -> Vec<String> {
    app_state::get_all_priority_keywords(app_state)
        .iter()
        .map(|pk| pk.keyword.clone())
        .collect()
}

/// 创建 SearchCore 实例
fn create_search_core(
    state: &app_state::AppState,
    include_clmclm: bool,
    include_others: bool,
) -> Result<SearchCore, String> {
    let (extraction_config, analysis_config) = build_llm_configs(state);
    let priority_keyword_strings = get_priority_keywords(state);
    let enabled_engines = get_active_engines(state);

    let clmclm_is_enabled_in_settings = enabled_engines.iter().any(|e| e.name == "clmclm.com");

    let custom_engine_tuples: Vec<(String, String)> = if include_others {
        enabled_engines
            .iter()
            .filter(|e| e.name != "clmclm.com")
            .map(|e| (e.name.clone(), e.url_template.clone()))
            .collect()
    } else {
        Vec::new()
    };

    let final_clmclm_status = include_clmclm && clmclm_is_enabled_in_settings;

    if custom_engine_tuples.is_empty() && !final_clmclm_status {
        return Err("No search engines available for this operation.".to_string());
    }

    println!(
        "🔧 Creating search core: Custom Engines: {}, CLMCLM: {}",
        custom_engine_tuples.len(),
        final_clmclm_status
    );

    Ok(searcher::create_ai_enhanced_search_core(
        extraction_config,
        analysis_config,
        priority_keyword_strings,
        custom_engine_tuples,
        final_clmclm_status,
    ))
}

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
            // 简化调试输出
            println!("[AI] Analyzed: '{}' -> '{}'", result.title, cleaned_title);

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
                error: None,
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
    let search_core = create_search_core(&state, true, true)?;
    search_core.search_multi_page(keyword.as_str(), pages).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_clmclm_first(
    state: tauri::State<'_, app_state::AppState>,
    keyword: String,
    max_pages: Option<u32>,
) -> Result<Vec<searcher::SearchResult>, String> {
    let pages = max_pages.unwrap_or(3);
    match create_search_core(&state, true, false) {
        Ok(search_core) => search_core.search_multi_page(keyword.as_str(), pages).await.map_err(|e| e.to_string()),
        Err(_) => Ok(Vec::new()), // 如果clmclm未启用，则返回空结果
    }
}

#[tauri::command]
async fn search_other_engines(
    state: tauri::State<'_, app_state::AppState>,
    keyword: String,
    max_pages: Option<u32>,
) -> Result<Vec<searcher::SearchResult>, String> {
    let pages = max_pages.unwrap_or(3);
    match create_search_core(&state, false, true) {
        Ok(search_core) => search_core.search_multi_page(keyword.as_str(), pages).await.map_err(|e| e.to_string()),
        Err(_) => Ok(Vec::new()), // 如果没有其他引擎，则返回空结果
    }
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
        batch_size: config.batch_size,
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
        batch_size: config.batch_size,
    };
    llm_service::test_connection(&llm_config).await.map_err(|e| e.to_string())
}

// 注意：load_llm_config_from_app 和 load_llm_config_from_file 函数已被删除
// 因为它们未被使用，LLM配置现在通过前端直接传递

// ============ LLM 配置相关命令 ============

#[tauri::command]
async fn get_llm_config(state: tauri::State<'_, app_state::AppState>) -> Result<app_state::LlmConfig, String> {
    let config = app_state::get_llm_config(&state);
    println!("🔧 Get LLM config: extraction_batch_size={}, analysis_batch_size={}", config.extraction_config.batch_size, config.analysis_config.batch_size);
    Ok(config)
}



#[tauri::command]
async fn batch_analyze_resources(
    state: tauri::State<'_, app_state::AppState>,
    results: Vec<searcher::SearchResult>,
) -> Result<Vec<llm_service::DetailedAnalysisResult>, String> {
    let config = app_state::get_llm_config(&state);

    println!("🔧 Frontend batch analysis: {} results, batch_size={}", results.len(), config.analysis_config.batch_size);

    if results.is_empty() {
        return Ok(Vec::new());
    }

    // 转换为批量分析格式
    let batch_items: Vec<llm_service::BatchAnalysisItem> = results
        .iter()
        .filter(|r| !r.file_list.is_empty())
        .map(|r| llm_service::BatchAnalysisItem {
            title: r.title.clone(),
            file_list: r.file_list.clone(),
        })
        .collect();

    if batch_items.is_empty() {
        println!("⚠️ No valid results with file lists for batch analysis");
        return Ok(Vec::new());
    }

    // 转换配置
    let llm_config = llm_service::LlmConfig {
        provider: config.analysis_config.provider,
        api_key: config.analysis_config.api_key,
        api_base: config.analysis_config.api_base,
        model: config.analysis_config.model,
        batch_size: config.analysis_config.batch_size,
    };

    let client = llm_service::GeminiClient::new();
    let batch_size = config.analysis_config.batch_size as usize;
    let mut all_results = Vec::new();
    let mut failed_batches = 0;
    const MAX_FAILED_BATCHES: usize = 3; // 最多允许3个批次失败

    // 分批处理
    for (batch_index, chunk) in batch_items.chunks(batch_size).enumerate() {
        println!("🔄 Frontend processing batch {}/{} ({} items)",
                 batch_index + 1,
                 (batch_items.len() + batch_size - 1) / batch_size,
                 chunk.len());

        // 如果失败的批次太多，直接返回错误
        if failed_batches >= MAX_FAILED_BATCHES {
            return Err(format!("Too many batch failures ({}/{}), aborting analysis",
                              failed_batches, MAX_FAILED_BATCHES));
        }

        match client.batch_analyze_multiple_items(chunk, &llm_config).await {
            Ok(batch_results) => {
                // 将批量结果转换为 DetailedAnalysisResult
                for (i, analysis_result) in batch_results.iter().enumerate() {
                    if let Some(original_result) = results.get(batch_index * batch_size + i) {
                        all_results.push(llm_service::DetailedAnalysisResult {
                            title: if analysis_result.cleaned_title.is_empty() {
                                clean_title_fallback(&original_result.title)
                            } else {
                                analysis_result.cleaned_title.clone()
                            },
                            purity_score: analysis_result.purity_score,
                            tags: analysis_result.tags.clone(),
                            magnet_link: original_result.magnet_link.clone(),
                            file_size: original_result.file_size.clone(),
                            file_list: original_result.file_list.clone(),
                            error: None,
                        });
                    }
                }
                println!("✅ Frontend batch {} success.", batch_index + 1);
            }
            Err(e) => {
                failed_batches += 1;
                println!("⚠️ Frontend batch {} failed ({}/{}): {}", batch_index + 1, failed_batches, MAX_FAILED_BATCHES, e);

                // 如果这是最后一次尝试，直接添加失败结果而不进行单个分析
                if failed_batches >= MAX_FAILED_BATCHES {
                    for (i, _item) in chunk.iter().enumerate() {
                        if let Some(original_result) = results.get(batch_index * batch_size + i) {
                            all_results.push(llm_service::DetailedAnalysisResult {
                                title: clean_title_fallback(&original_result.title),
                                purity_score: 50, // 默认分数
                                tags: vec!["Analysis Failed - Too Many Failures".to_string()],
                                magnet_link: original_result.magnet_link.clone(),
                                file_size: original_result.file_size.clone(),
                                file_list: original_result.file_list.clone(),
                                error: Some("Too many batch failures, analysis aborted".to_string()),
                            });
                        }
                    }
                    continue;
                }

                // 回退到单个分析（使用批量分析处理单个项目）
                for (i, item) in chunk.iter().enumerate() {
                    if let Some(original_result) = results.get(batch_index * batch_size + i) {
                        // 将单个项目包装为批量格式
                        let single_item = vec![item.clone()];

                        // 单个分析只尝试一次，不进行重试
                        match tokio::time::timeout(
                            std::time::Duration::from_secs(30), // 30秒超时
                            client.batch_analyze_multiple_items(&single_item, &llm_config)
                        ).await {
                            Ok(Ok(mut batch_results)) => {
                                if let Some(result) = batch_results.pop() {
                                    all_results.push(llm_service::DetailedAnalysisResult {
                                        title: if result.cleaned_title.is_empty() {
                                            clean_title_fallback(&original_result.title)
                                        } else {
                                            result.cleaned_title
                                        },
                                        purity_score: result.purity_score,
                                        tags: result.tags,
                                        magnet_link: original_result.magnet_link.clone(),
                                        file_size: original_result.file_size.clone(),
                                        file_list: original_result.file_list.clone(),
                                        error: None,
                                    });
                                } else {
                                    println!("⚠️ Individual analysis for '{}' returned no results", item.title);
                                    all_results.push(llm_service::DetailedAnalysisResult {
                                        title: clean_title_fallback(&original_result.title),
                                        purity_score: 50,
                                        tags: vec!["No Results".to_string()],
                                        magnet_link: original_result.magnet_link.clone(),
                                        file_size: original_result.file_size.clone(),
                                        file_list: original_result.file_list.clone(),
                                        error: Some("Individual analysis returned no results".to_string()),
                                    });
                                }
                            }
                            Ok(Err(individual_error)) => {
                                println!("⚠️ Individual analysis for '{}' failed: {}", item.title, individual_error);
                                all_results.push(llm_service::DetailedAnalysisResult {
                                    title: clean_title_fallback(&original_result.title),
                                    purity_score: 50,
                                    tags: vec!["Individual Analysis Failed".to_string()],
                                    magnet_link: original_result.magnet_link.clone(),
                                    file_size: original_result.file_size.clone(),
                                    file_list: original_result.file_list.clone(),
                                    error: Some(format!("Individual analysis failed: {}", individual_error)),
                                });
                            }
                            Err(_timeout) => {
                                println!("⚠️ Individual analysis for '{}' timed out", item.title);
                                all_results.push(llm_service::DetailedAnalysisResult {
                                    title: clean_title_fallback(&original_result.title),
                                    purity_score: 50,
                                    tags: vec!["Analysis Timeout".to_string()],
                                    magnet_link: original_result.magnet_link.clone(),
                                    file_size: original_result.file_size.clone(),
                                    file_list: original_result.file_list.clone(),
                                    error: Some("Analysis timed out after 30 seconds".to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    println!("🎉 Frontend batch analysis completed: {} results processed", all_results.len());
    Ok(all_results)
}

#[tauri::command]
async fn update_llm_config(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, app_state::AppState>,
    config: app_state::LlmConfig,
) -> Result<(), String> {
    println!("🔧 Updating LLM config: extraction_batch_size={}, analysis_batch_size={}", config.extraction_config.batch_size, config.analysis_config.batch_size);

    app_state::update_llm_config(&state, config).map_err(|e| e.to_string())?;

    // 保存状态到文件
    app_state::save_app_state(&app_handle, &state).map_err(|e| e.to_string())?;

    println!("🔧 LLM config saved.");
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
            search_clmclm_first,
            search_other_engines,
            test_connection,
            test_extraction_connection,
            test_analysis_connection,
            analyze_resource,
            batch_analyze_resources,
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
