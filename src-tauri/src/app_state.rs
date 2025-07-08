// src-tauri/src/app_state.rs

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use anyhow::{Result, anyhow};
use uuid::Uuid;

/// 收藏项数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItem {
    pub id: String,
    pub title: String,
    pub magnet_link: String,
    pub file_size: Option<String>,
    pub file_list: Vec<String>,
    pub created_at: String, // ISO 8601 格式
}

/// 搜索引擎配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub id: String,
    pub name: String,
    pub url_template: String, // 包含 {keyword} 和 {page} 占位符
    pub is_enabled: bool,
    pub is_deletable: bool, // 默认引擎不可删除
}

/// 优先关键词
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityKeyword {
    pub id: String,
    pub keyword: String,
}

/// 单个LLM配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleLlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

impl Default for SingleLlmConfig {
    fn default() -> Self {
        Self {
            provider: "gemini".to_string(),
            api_key: "".to_string(),
            api_base: "https://generativelanguage.googleapis.com".to_string(),
            model: "gemini-2.5-flash-lite-preview-06-17".to_string(),
        }
    }
}

/// 双LLM配置 - 分别用于第一次和第二次API调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub extraction_config: SingleLlmConfig,  // 第一次API调用：从HTML提取基础信息
    pub analysis_config: SingleLlmConfig,    // 第二次API调用：分析分数和标签
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            extraction_config: SingleLlmConfig::default(),
            analysis_config: SingleLlmConfig::default(),
        }
    }
}

/// 搜索设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSettings {
    pub use_smart_filter: bool,
    pub max_pages: u32,
    pub sort_by: String,
    pub title_must_contain_keyword: bool,
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            use_smart_filter: true,
            max_pages: 1,
            sort_by: "score".to_string(),
            title_must_contain_keyword: true,
        }
    }
}

/// 应用状态数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub favorites: Vec<FavoriteItem>,
    pub search_engines: Vec<SearchEngine>,
    pub priority_keywords: Vec<PriorityKeyword>,
    pub llm_config: LlmConfig,
    pub search_settings: SearchSettings,
    pub version: String, // 用于数据迁移
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            favorites: Vec::new(),
            search_engines: vec![
                // 默认搜索引擎
                SearchEngine {
                    id: "default_clmclm".to_string(),
                    name: "clmclm.com".to_string(),
                    url_template: "http://clmclm.com/search-{keyword}-1-1-{page}.html".to_string(),
                    is_enabled: true,
                    is_deletable: false,
                }
            ],
            priority_keywords: Vec::new(),
            llm_config: LlmConfig::default(),
            search_settings: SearchSettings::default(),
            version: "1.0.0".to_string(),
        }
    }
}

/// 应用状态管理器
pub struct AppStateManager {
    data_file_path: PathBuf,
}

impl AppStateManager {
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| anyhow!("Failed to get app data directory: {}", e))?;
        
        // 确保目录存在
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| anyhow!("Failed to create app data directory: {}", e))?;
        
        let data_file_path = app_data_dir.join("app_data.json");
        
        Ok(Self { data_file_path })
    }

    /// 加载应用数据
    pub fn load_data(&self) -> Result<AppData> {
        if !self.data_file_path.exists() {
            // 文件不存在，返回默认数据并保存
            let default_data = AppData::default();
            self.save_data(&default_data)?;
            return Ok(default_data);
        }

        let content = fs::read_to_string(&self.data_file_path)
            .map_err(|e| anyhow!("Failed to read app data file: {}", e))?;
        
        let data: AppData = match serde_json::from_str(&content) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse app data, using default: {}", e);
                // 如果解析失败，备份损坏的文件并使用默认数据
                let backup_path = self.data_file_path.with_extension("json.backup");
                let _ = fs::copy(&self.data_file_path, backup_path);

                let default_data = AppData::default();
                let _ = self.save_data(&default_data);
                default_data
            }
        };

        Ok(data)
    }

    /// 保存应用数据
    pub fn save_data(&self, data: &AppData) -> Result<()> {
        let content = serde_json::to_string_pretty(data)
            .map_err(|e| anyhow!("Failed to serialize app data: {}", e))?;
        
        fs::write(&self.data_file_path, content)
            .map_err(|e| anyhow!("Failed to write app data file: {}", e))?;
        
        Ok(())
    }
}

/// Tauri 状态管理
pub type AppState = std::sync::Mutex<AppData>;

/// 初始化应用状态
pub fn init_app_state(app_handle: &AppHandle) -> Result<AppState> {
    let manager = AppStateManager::new(app_handle)?;
    let data = manager.load_data()?;
    Ok(std::sync::Mutex::new(data))
}

/// 保存当前状态到文件
pub fn save_app_state(app_handle: &AppHandle, state: &AppState) -> Result<()> {
    let manager = AppStateManager::new(app_handle)?;
    let data = state.lock().unwrap().clone();
    manager.save_data(&data)
}

// ============ 收藏夹相关函数 ============

/// 添加到收藏夹
pub fn add_to_favorites(
    state: &AppState,
    title: String,
    magnet_link: String,
    file_size: Option<String>,
    file_list: Vec<String>,
) -> Result<FavoriteItem> {
    let mut data = state.lock().unwrap();
    
    // 检查是否已经收藏
    if data.favorites.iter().any(|item| item.magnet_link == magnet_link) {
        return Err(anyhow!("Item already in favorites"));
    }
    
    let favorite_item = FavoriteItem {
        id: Uuid::new_v4().to_string(),
        title,
        magnet_link,
        file_size,
        file_list,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    data.favorites.push(favorite_item.clone());
    Ok(favorite_item)
}

/// 获取所有收藏
pub fn get_all_favorites(state: &AppState) -> Vec<FavoriteItem> {
    let data = state.lock().unwrap();
    data.favorites.clone()
}

/// 从收藏夹移除
pub fn remove_from_favorites(state: &AppState, id: String) -> Result<()> {
    let mut data = state.lock().unwrap();
    let initial_len = data.favorites.len();
    data.favorites.retain(|item| item.id != id);
    
    if data.favorites.len() == initial_len {
        return Err(anyhow!("Favorite item not found"));
    }
    
    Ok(())
}

/// 在收藏中搜索
pub fn search_favorites(state: &AppState, query: String) -> Vec<FavoriteItem> {
    let data = state.lock().unwrap();
    let query_lower = query.to_lowercase();
    
    data.favorites
        .iter()
        .filter(|item| item.title.to_lowercase().contains(&query_lower))
        .cloned()
        .collect()
}

// ============ 搜索引擎相关函数 ============

/// 添加搜索引擎
pub fn add_search_engine(
    state: &AppState,
    name: String,
    url_template: String,
) -> Result<SearchEngine> {
    let mut data = state.lock().unwrap();
    
    let engine = SearchEngine {
        id: Uuid::new_v4().to_string(),
        name,
        url_template,
        is_enabled: true,
        is_deletable: true,
    };
    
    data.search_engines.push(engine.clone());
    Ok(engine)
}

/// 获取所有搜索引擎
pub fn get_all_engines(state: &AppState) -> Vec<SearchEngine> {
    let data = state.lock().unwrap();
    data.search_engines.clone()
}

/// 更新搜索引擎状态
pub fn update_engine_status(state: &AppState, id: String, is_enabled: bool) -> Result<()> {
    let mut data = state.lock().unwrap();
    
    if let Some(engine) = data.search_engines.iter_mut().find(|e| e.id == id) {
        engine.is_enabled = is_enabled;
        Ok(())
    } else {
        Err(anyhow!("Search engine not found"))
    }
}

/// 删除搜索引擎
pub fn delete_engine(state: &AppState, id: String) -> Result<()> {
    let mut data = state.lock().unwrap();
    
    // 检查是否可删除
    if let Some(engine) = data.search_engines.iter().find(|e| e.id == id) {
        if !engine.is_deletable {
            return Err(anyhow!("Cannot delete default search engine"));
        }
    }
    
    let initial_len = data.search_engines.len();
    data.search_engines.retain(|engine| engine.id != id);
    
    if data.search_engines.len() == initial_len {
        return Err(anyhow!("Search engine not found"));
    }
    
    Ok(())
}

// ============ 优先关键词相关函数 ============

/// 添加优先关键词
pub fn add_priority_keyword(state: &AppState, keyword: String) -> Result<PriorityKeyword> {
    let mut data = state.lock().unwrap();
    
    // 检查是否已存在
    if data.priority_keywords.iter().any(|k| k.keyword == keyword) {
        return Err(anyhow!("Keyword already exists"));
    }
    
    let priority_keyword = PriorityKeyword {
        id: Uuid::new_v4().to_string(),
        keyword,
    };
    
    data.priority_keywords.push(priority_keyword.clone());
    Ok(priority_keyword)
}

/// 获取所有优先关键词
pub fn get_all_priority_keywords(state: &AppState) -> Vec<PriorityKeyword> {
    let data = state.lock().unwrap();
    data.priority_keywords.clone()
}

/// 删除优先关键词
pub fn delete_priority_keyword(state: &AppState, id: String) -> Result<()> {
    let mut data = state.lock().unwrap();
    let initial_len = data.priority_keywords.len();
    data.priority_keywords.retain(|keyword| keyword.id != id);
    
    if data.priority_keywords.len() == initial_len {
        return Err(anyhow!("Priority keyword not found"));
    }
    
    Ok(())
}

// ============ LLM 配置相关函数 ============

/// 获取 LLM 配置
pub fn get_llm_config(state: &AppState) -> LlmConfig {
    let data = state.lock().unwrap();
    data.llm_config.clone()
}

/// 更新 LLM 配置
pub fn update_llm_config(state: &AppState, config: LlmConfig) -> Result<()> {
    let mut data = state.lock().unwrap();
    data.llm_config = config;
    Ok(())
}

/// 获取提取配置（第一次API调用）
pub fn get_extraction_config(state: &AppState) -> SingleLlmConfig {
    let data = state.lock().unwrap();
    data.llm_config.extraction_config.clone()
}

/// 获取分析配置（第二次API调用）
pub fn get_analysis_config(state: &AppState) -> SingleLlmConfig {
    let data = state.lock().unwrap();
    data.llm_config.analysis_config.clone()
}

/// 更新提取配置
pub fn update_extraction_config(state: &AppState, config: SingleLlmConfig) -> Result<()> {
    let mut data = state.lock().unwrap();
    data.llm_config.extraction_config = config;
    Ok(())
}

/// 更新分析配置
pub fn update_analysis_config(state: &AppState, config: SingleLlmConfig) -> Result<()> {
    let mut data = state.lock().unwrap();
    data.llm_config.analysis_config = config;
    Ok(())
}

// ============ 搜索设置相关函数 ============

/// 获取搜索设置
pub fn get_search_settings(state: &AppState) -> SearchSettings {
    let data = state.lock().unwrap();
    data.search_settings.clone()
}

/// 更新搜索设置
pub fn update_search_settings(state: &AppState, settings: SearchSettings) -> Result<()> {
    let mut data = state.lock().unwrap();
    data.search_settings = settings;
    Ok(())
}
