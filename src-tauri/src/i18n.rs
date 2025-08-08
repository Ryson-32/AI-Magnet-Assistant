// src-tauri/src/i18n.rs

use anyhow::{Result, anyhow};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 错误代码枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorCode {
    // 搜索相关错误
    SearchNoEngines,
    SearchTimeout,
    SearchFailed(String),
    
    // 收藏相关错误
    FavoritesDuplicate,
    FavoritesNotFound,
    FavoritesQuotaExceeded,
    
    // 搜索引擎相关错误
    EngineNotFound,
    EngineNotDeletable,
    EngineInvalid,
    
    // 系统相关错误
    SystemIOError,
    SystemPermissionDenied,
    SystemNetworkError,
    
    // AI服务相关错误
    AIServiceUnavailable,
    AIServiceQuotaExceeded,
    AIServiceInvalidKey,
    
    // 未知错误
    UnknownError(String),
}

impl ErrorCode {
    /// 将错误代码转换为字符串标识
    #[allow(dead_code)]
    pub fn to_code_string(&self) -> String {
        match self {
            ErrorCode::SearchNoEngines => "ERR_SEARCH_NO_ENGINES".to_string(),
            ErrorCode::SearchTimeout => "ERR_SEARCH_TIMEOUT".to_string(),
            ErrorCode::SearchFailed(_) => "ERR_SEARCH_FAILED".to_string(),
            ErrorCode::FavoritesDuplicate => "ERR_FAVORITES_DUPLICATE".to_string(),
            ErrorCode::FavoritesNotFound => "ERR_FAVORITES_NOT_FOUND".to_string(),
            ErrorCode::FavoritesQuotaExceeded => "ERR_FAVORITES_QUOTA_EXCEEDED".to_string(),
            ErrorCode::EngineNotFound => "ERR_ENGINE_NOT_FOUND".to_string(),
            ErrorCode::EngineNotDeletable => "ERR_ENGINE_NOT_DELETABLE".to_string(),
            ErrorCode::EngineInvalid => "ERR_ENGINE_INVALID".to_string(),
            ErrorCode::SystemIOError => "ERR_SYSTEM_IO_ERROR".to_string(),
            ErrorCode::SystemPermissionDenied => "ERR_SYSTEM_PERMISSION_DENIED".to_string(),
            ErrorCode::SystemNetworkError => "ERR_SYSTEM_NETWORK_ERROR".to_string(),
            ErrorCode::AIServiceUnavailable => "ERR_AI_SERVICE_UNAVAILABLE".to_string(),
            ErrorCode::AIServiceQuotaExceeded => "ERR_AI_SERVICE_QUOTA_EXCEEDED".to_string(),
            ErrorCode::AIServiceInvalidKey => "ERR_AI_SERVICE_INVALID_KEY".to_string(),
            ErrorCode::UnknownError(_) => "ERR_UNKNOWN_ERROR".to_string(),
        }
    }
    
    /// 获取错误参数
    pub fn get_params(&self) -> Option<HashMap<String, String>> {
        match self {
            ErrorCode::SearchFailed(details) => {
                let mut params = HashMap::new();
                params.insert("details".to_string(), details.clone());
                Some(params)
            }
            ErrorCode::UnknownError(code) => {
                let mut params = HashMap::new();
                params.insert("code".to_string(), code.clone());
                Some(params)
            }
            _ => None,
        }
    }
}

/// 前端错误结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendError {
    pub code: String,
    pub params: Option<HashMap<String, String>>,
}

impl ErrorCode {
    /// 转换为前端可用的错误格式
    #[allow(dead_code)]
    pub fn to_frontend_error(&self) -> FrontendError {
        FrontendError {
            code: self.to_code_string(),
            params: self.get_params(),
        }
    }
}

/// 国际化消息结构
#[derive(Debug, Clone, Deserialize)]
pub struct Messages {
    pub errors: HashMap<String, String>,
    pub system: HashMap<String, String>,
}

/// 国际化管理器
#[derive(Debug)]
pub struct I18nManager {
    current_locale: Arc<Mutex<String>>,
    messages: Arc<Mutex<HashMap<String, Messages>>>,
    supported_locales: Vec<String>,
}

impl I18nManager {
    /// 创建新的国际化管理器
    pub fn new() -> Self {
        let manager = Self {
            current_locale: Arc::new(Mutex::new("en".to_string())),
            messages: Arc::new(Mutex::new(HashMap::new())),
            supported_locales: vec!["en".to_string(), "zh-CN".to_string()],
        };
        
        // 初始化时加载默认语言包
        if let Err(e) = manager.load_locale("en") {
            eprintln!("警告: 无法加载默认语言包: {e}");
        }
        
        manager
    }
    
    /// 加载指定语言包
    pub fn load_locale(&self, locale: &str) -> Result<()> {
        let messages_json = match locale {
            "en" => include_str!("../locales/en/messages.json"),
            "zh-CN" => include_str!("../locales/zh-CN/messages.json"),
            _ => return Err(anyhow!("Unsupported locale: {}", locale)),
        };
        
        let messages: Messages = serde_json::from_str(messages_json)
            .map_err(|e| anyhow!("Failed to parse messages for locale {}: {}", locale, e))?;
        
        let mut messages_map = self.messages.lock().unwrap();
        messages_map.insert(locale.to_string(), messages);
        
        Ok(())
    }
    
    /// 设置当前语言
    pub fn set_locale(&self, locale: &str) -> Result<()> {
        if !self.supported_locales.contains(&locale.to_string()) {
            return Err(anyhow!("Unsupported locale: {}", locale));
        }
        
        // 如果语言包未加载，先加载它
        {
            let messages_map = self.messages.lock().unwrap();
            if !messages_map.contains_key(locale) {
                drop(messages_map);
                self.load_locale(locale)?;
            }
        }
        
        let mut current_locale = self.current_locale.lock().unwrap();
        *current_locale = locale.to_string();
        
        println!("📝 语言已切换到: {locale}");
        Ok(())
    }
    
    /// 获取当前语言
    pub fn get_current_locale(&self) -> String {
        let current_locale = self.current_locale.lock().unwrap();
        current_locale.clone()
    }
    
    /// 获取支持的语言列表
    pub fn get_supported_locales(&self) -> Vec<String> {
        self.supported_locales.clone()
    }
    
    /// 翻译消息键
    pub fn translate(&self, key: &str, params: Option<&HashMap<String, String>>) -> String {
        let current_locale = self.get_current_locale();
        self.translate_with_locale(key, &current_locale, params)
    }
    
    /// 使用指定语言翻译消息键
    pub fn translate_with_locale(&self, key: &str, locale: &str, params: Option<&HashMap<String, String>>) -> String {
        let messages_map = self.messages.lock().unwrap();
        
        let messages = match messages_map.get(locale) {
            Some(messages) => messages,
            None => {
                // 如果找不到指定语言，回退到英文
                match messages_map.get("en") {
                    Some(messages) => messages,
                    None => return key.to_string(), // 如果连英文都没有，返回键本身
                }
            }
        };
        
        // 尝试从不同的消息类别中查找
        let message = if key.starts_with("errors.") {
            let error_key = key.strip_prefix("errors.").unwrap();
            messages.errors.get(error_key)
        } else if key.starts_with("system.") {
            let system_key = key.strip_prefix("system.").unwrap();
            messages.system.get(system_key)
        } else {
            // 直接在errors和system中查找
            messages.errors.get(key).or_else(|| messages.system.get(key))
        };
        
        match message {
            Some(msg) => self.substitute_params(msg, params),
            None => {
                eprintln!("警告: 未找到翻译键 '{key}' (语言: {locale})");
                key.to_string()
            }
        }
    }
    
    /// 替换消息中的参数占位符
    fn substitute_params(&self, message: &str, params: Option<&HashMap<String, String>>) -> String {
        let Some(params) = params else {
            return message.to_string();
        };
        
        let mut result = message.to_string();
        for (key, value) in params {
            let placeholder = format!("{{{key}}}");
            result = result.replace(&placeholder, value);
        }
        result
    }
    
    /// 翻译错误代码
    pub fn translate_error_code(&self, error_code: &ErrorCode) -> String {
        let key = match error_code {
            ErrorCode::SearchNoEngines => "errors.search_no_engines",
            ErrorCode::SearchTimeout => "errors.search_timeout",
            ErrorCode::SearchFailed(_) => "errors.search_failed",
            ErrorCode::FavoritesDuplicate => "errors.favorites_duplicate",
            ErrorCode::FavoritesNotFound => "errors.favorites_not_found",
            ErrorCode::FavoritesQuotaExceeded => "errors.favorites_quota_exceeded",
            ErrorCode::EngineNotFound => "errors.engine_not_found",
            ErrorCode::EngineNotDeletable => "errors.engine_not_deletable",
            ErrorCode::EngineInvalid => "errors.engine_invalid",
            ErrorCode::SystemIOError => "errors.system_io_error",
            ErrorCode::SystemPermissionDenied => "errors.system_permission_denied",
            ErrorCode::SystemNetworkError => "errors.system_network_error",
            ErrorCode::AIServiceUnavailable => "errors.ai_service_unavailable",
            ErrorCode::AIServiceQuotaExceeded => "errors.ai_service_quota_exceeded",
            ErrorCode::AIServiceInvalidKey => "errors.ai_service_invalid_key",
            ErrorCode::UnknownError(_) => "errors.unknown_error",
        };
        
        self.translate(key, error_code.get_params().as_ref())
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局国际化管理器实例
static I18N_MANAGER: Lazy<I18nManager> = Lazy::new(|| {
    let manager = I18nManager::new();
    
    // 尝试自动检测系统语言
    if let Some(system_locale) = sys_locale::get_locale() {
        let locale = normalize_locale(&system_locale);
        if let Err(e) = manager.set_locale(&locale) {
            eprintln!("警告: 无法设置系统语言 '{locale}': {e}, 使用默认语言 'en'");
        }
    }
    
    manager
});

/// 标准化语言代码
fn normalize_locale(locale: &str) -> String {
    match locale {
        l if l.starts_with("zh") => "zh-CN".to_string(),
        l if l.starts_with("en") => "en".to_string(),
        _ => "en".to_string(), // 默认回退到英文
    }
}

/// 获取全局国际化管理器
pub fn get_i18n_manager() -> &'static I18nManager {
    &I18N_MANAGER
}

/// 便捷的翻译函数
#[allow(dead_code)]
pub fn t(key: &str) -> String {
    get_i18n_manager().translate(key, None)
}

/// 带参数的便捷翻译函数
#[allow(dead_code)]
pub fn t_with_params(key: &str, params: &HashMap<String, String>) -> String {
    get_i18n_manager().translate(key, Some(params))
}

/// 翻译错误代码的便捷函数
pub fn translate_error(error_code: &ErrorCode) -> String {
    get_i18n_manager().translate_error_code(error_code)
}

/// Tauri 命令：获取系统语言
#[tauri::command]
pub async fn get_system_locale() -> Result<String, String> {
    match sys_locale::get_locale() {
        Some(locale) => Ok(normalize_locale(&locale)),
        None => Ok("en".to_string()),
    }
}

/// Tauri 命令：设置应用语言
#[tauri::command]
pub async fn set_app_locale(locale: String) -> Result<(), String> {
    get_i18n_manager()
        .set_locale(&locale)
        .map_err(|e| e.to_string())
}

/// Tauri 命令：获取当前语言
#[tauri::command]
pub async fn get_current_locale() -> Result<String, String> {
    Ok(get_i18n_manager().get_current_locale())
}

/// Tauri 命令：获取支持的语言列表
#[tauri::command]
pub async fn get_supported_locales() -> Result<Vec<String>, String> {
    Ok(get_i18n_manager().get_supported_locales())
}

/// Tauri 命令：翻译消息键
#[tauri::command]
pub async fn get_localized_message(key: String, params: Option<HashMap<String, String>>) -> Result<String, String> {
    Ok(get_i18n_manager().translate(&key, params.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_to_string() {
        let error = ErrorCode::SearchNoEngines;
        assert_eq!(error.to_code_string(), "ERR_SEARCH_NO_ENGINES");
        
        let error_with_params = ErrorCode::SearchFailed("Network timeout".to_string());
        assert_eq!(error_with_params.to_code_string(), "ERR_SEARCH_FAILED");
        assert!(error_with_params.get_params().is_some());
    }

    #[test]
    fn test_i18n_manager() {
        let manager = I18nManager::new();
        
        // 测试默认语言
        assert_eq!(manager.get_current_locale(), "en");
        
        // 测试翻译
        let message = manager.translate("errors.search_no_engines", None);
        assert!(!message.is_empty());
        
        // 测试参数替换
        let mut params = HashMap::new();
        params.insert("details".to_string(), "Connection failed".to_string());
        let message_with_params = manager.translate("errors.search_failed", Some(&params));
        assert!(message_with_params.contains("Connection failed"));
    }

    #[test]
    fn test_normalize_locale() {
        assert_eq!(normalize_locale("zh-CN"), "zh-CN");
        assert_eq!(normalize_locale("zh"), "zh-CN");
        assert_eq!(normalize_locale("en-US"), "en");
        assert_eq!(normalize_locale("fr"), "en");
    }
}