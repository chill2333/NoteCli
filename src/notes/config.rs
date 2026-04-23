use std::{fs, path::PathBuf};
use serde::Deserialize;
use toml;

/// 主配置结构
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_general")]
    pub general: GeneralConfig,
    #[serde(default = "default_display")]
    pub display: DisplayConfig,
    #[serde(default = "default_storage")]
    pub storage: StorageConfig,
    #[serde(default = "default_search")]
    pub search: SearchConfig,
}

/// [general] 段
#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_editor")]
    pub default_editor: String,
    #[serde(default = "default_priority")]
    pub default_priority: String,
    #[serde(default = "default_category")]
    pub default_category: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_pager")]
    pub pager: String,
}

/// [display] 段
#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_color")]
    pub color: bool,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default = "default_table_style")]
    pub table_style: String,
    #[serde(default = "default_title_max_width")]
    pub title_max_width: usize,
}

/// [storage] 段
#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_notes_dir")]
    pub notes_dir: PathBuf,
    #[serde(default = "default_index_file")]
    pub index_file: PathBuf,
}

/// [search] 段
#[derive(Debug, Deserialize)]
pub struct SearchConfig {
    #[serde(default = "default_search_mode")]
    pub default_mode: String,
    #[serde(default = "default_case_sensitive")]
    pub case_sensitive: bool,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

// ========== 字段级别的默认值函数 ==========
fn default_editor() -> String { "vim".to_string() }
fn default_priority() -> String { "normal".to_string() }
fn default_category() -> String { "default".to_string() }
fn default_language() -> String { "zh-CN".to_string() }
fn default_pager() -> String { "less".to_string() }

fn default_color() -> bool { true }
fn default_date_format() -> String { "%Y-%m-%d %H:%M".to_string() }
fn default_table_style() -> String { "compact".to_string() }
fn default_title_max_width() -> usize { 50 }

fn default_notes_dir() -> PathBuf {
    PathBuf::from("./.notecli/notes")
}
fn default_index_file() -> PathBuf {
    PathBuf::from("./.notecli/index.json")
}

fn default_search_mode() -> String { "plain".to_string() }
fn default_case_sensitive() -> bool { false }
fn default_max_results() -> usize { 50 }

// ========== 子配置段的默认函数（组合字段默认值） ==========
fn default_general() -> GeneralConfig {
    GeneralConfig {
        default_editor: default_editor(),
        default_priority: default_priority(),
        default_category: default_category(),
        language: default_language(),
        pager: default_pager(),
    }
}

fn default_display() -> DisplayConfig {
    DisplayConfig {
        color: default_color(),
        date_format: default_date_format(),
        table_style: default_table_style(),
        title_max_width: default_title_max_width(),
    }
}

fn default_storage() -> StorageConfig {
    StorageConfig {
        notes_dir: default_notes_dir(),
        index_file: default_index_file(),
    }
}

fn default_search() -> SearchConfig {
    SearchConfig {
        default_mode: default_search_mode(),
        case_sensitive: default_case_sensitive(),
        max_results: default_max_results(),
    }
}

impl Config {
    /// 从指定的 TOML 文件加载配置
    /// 
    /// # 参数
    /// - `path_opt`: 可选的配置文件路径，为 `None` 时使用默认路径 `"./src/config.toml"`
    pub fn from_file(path_opt: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_str = path_opt.unwrap_or("./src/config.toml");
        
        let path = match fs::canonicalize(path_str) {
            Ok(abs_path) => {
                println!("配置文件路径有效: {}", abs_path.display());
                abs_path
            }
            Err(e) => {
                eprintln!("配置文件路径无效: {} - {}", path_str, e);
                return Err(Box::new(e));
            }
        };

        let content = match fs::read_to_string(&path) {
            Ok(data) => {
                println!("成功读取配置文件: {}", path.display());
                data
            }
            Err(e) => {
                eprintln!("读取配置文件失败: {} - {}", path.display(), e);
                return Err(Box::new(e));
            }
        };

        let config: Config = match toml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("解析 TOML 失败: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(config)
    }
}