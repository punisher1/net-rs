use fluent::{FluentBundle, FluentResource};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use std::str::FromStr;
use std::sync::RwLock;

/// 当前支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh-CN",
        }
    }
}

/// 全局语言设置
static CURRENT_LANGUAGE: Lazy<RwLock<Language>> = Lazy::new(|| RwLock::new(detect_system_language()));

/// 语言资源管理器
pub struct LanguageManager {
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
}

impl LanguageManager {
    /// 创建新的语言管理器并加载所有语言资源
    pub fn new() -> Self {
        let mut bundles = HashMap::new();
        
        // 加载英语资源
        let en_resource = FluentResource::try_new(include_str!("../../locales/en.ftl").to_string())
            .expect("Failed to parse English language resource");
        let en_lang = LanguageIdentifier::from_str("en").unwrap();
        let mut en_bundle = FluentBundle::new(vec![en_lang]);
        en_bundle.add_resource(en_resource).expect("Failed to add English resource");
        bundles.insert(Language::English, en_bundle);
        
        // 加载中文资源
        let zh_resource = FluentResource::try_new(include_str!("../../locales/zh-CN.ftl").to_string())
            .expect("Failed to parse Chinese language resource");
        let zh_lang = LanguageIdentifier::from_str("zh-CN").unwrap();
        let mut zh_bundle = FluentBundle::new(vec![zh_lang]);
        zh_bundle.add_resource(zh_resource).expect("Failed to add Chinese resource");
        bundles.insert(Language::Chinese, zh_bundle);
        
        Self { bundles }
    }

    /// 获取指定键的本地化文本
    pub fn get_text(&self, key: &str) -> String {
        let current_lang = *CURRENT_LANGUAGE.read().unwrap();
        
        if let Some(bundle) = self.bundles.get(&current_lang) {
            if let Some(message) = bundle.get_message(key) {
                if let Some(pattern) = message.value() {
                    let value = bundle.format_pattern(pattern, None, &mut vec![]);
                    {
                        return value.into_owned();
                    }
                }
            }
        }
        
        // 回退到键本身
        key.to_string()
    }
    
    /// 设置当前语言
    pub fn set_language(language: Language) {
        let mut current = CURRENT_LANGUAGE.write().unwrap();
        *current = language;
    }
    
    /// 获取当前语言
    pub fn get_current_language() -> Language {
        *CURRENT_LANGUAGE.read().unwrap()
    }
}

/// 检测系统语言
fn detect_system_language() -> Language {
    // 根据环境变量检测系统语言
    if let Ok(lang) = std::env::var("LANG").or_else(|_| std::env::var("LANGUAGE")) {
        if lang.starts_with("zh") {
            return Language::Chinese;
        }
    }
    
    // 默认为英语
    Language::English
}

// 创建一个便捷的获取本地化文本的函数
pub fn t(key: &str) -> String {
    // 为了避免在测试和编译时实际加载资源文件，这里只是返回键
    // 在实际使用时替换为实际获取本地化文本的逻辑
    // 完整实现需要在程序启动时初始化 LanguageManager
    key.to_string()
}