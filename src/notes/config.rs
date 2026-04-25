use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use toml;

/// 默认配置文件路径
pub fn default_config_path() -> PathBuf {
    PathBuf::from("./.notecli/config.toml")
}

/// 主配置结构
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_general")]
    pub general: GeneralConfig,
    #[serde(default = "default_display")]
    pub display: DisplayConfig,
    #[serde(default = "default_storage")]
    pub storage: StorageConfig,
    #[serde(default = "default_search")]
    pub search: SearchConfig,
    #[serde(default = "default_theme")]
    pub theme: ThemeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: default_general(),
            display: default_display(),
            storage: default_storage(),
            search: default_search(),
            theme: default_theme(),
        }
    }
}

/// [general] 段
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    #[serde(default = "default_color")]
    pub color: bool,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default = "default_title_max_width")]
    pub title_max_width: usize,
}

/// [storage] 段
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageConfig {
    #[serde(default = "default_notes_dir")]
    pub notes_dir: PathBuf,
    #[serde(default = "default_index_file")]
    pub index_file: PathBuf,
}

/// [search] 段
#[derive(Debug, Deserialize, Serialize)]
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

/// [theme] 段
#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_title")]
    pub title: String,
    #[serde(default = "default_theme_id")]
    pub id: String,
    #[serde(default = "default_theme_tag")]
    pub tag: String,
    #[serde(default = "default_theme_category")]
    pub category: String,
    #[serde(default = "default_theme_priority_low")]
    pub priority_low: String,
    #[serde(default = "default_theme_priority_normal")]
    pub priority_normal: String,
    #[serde(default = "default_theme_priority_high")]
    pub priority_high: String,
    #[serde(default = "default_theme_priority_urgent")]
    pub priority_urgent: String,
    #[serde(default = "default_theme_separator")]
    pub separator: String,
    #[serde(default = "default_theme_date")]
    pub date: String,
}

fn default_theme_title() -> String { "cyan bold".to_string() }
fn default_theme_id() -> String { "yellow".to_string() }
fn default_theme_tag() -> String { "green".to_string() }
fn default_theme_category() -> String { "blue".to_string() }
fn default_theme_priority_low() -> String { "white".to_string() }
fn default_theme_priority_normal() -> String { "green".to_string() }
fn default_theme_priority_high() -> String { "yellow bold".to_string() }
fn default_theme_priority_urgent() -> String { "red bold".to_string() }
fn default_theme_separator() -> String { "dark_gray".to_string() }
fn default_theme_date() -> String { "dark_gray".to_string() }

fn default_theme() -> ThemeConfig {
    ThemeConfig {
        title: default_theme_title(),
        id: default_theme_id(),
        tag: default_theme_tag(),
        category: default_theme_category(),
        priority_low: default_theme_priority_low(),
        priority_normal: default_theme_priority_normal(),
        priority_high: default_theme_priority_high(),
        priority_urgent: default_theme_priority_urgent(),
        separator: default_theme_separator(),
        date: default_theme_date(),
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
                // println!("配置文件路径有效: {}", abs_path.display());
                abs_path
            }
            Err(e) => {
                eprintln!("配置文件路径无效: {} - {}", path_str, e);
                return Err(Box::new(e));
            }
        };

        let content = match fs::read_to_string(&path) {
            Ok(data) => {
                // println!("成功读取配置文件: {}", path.display());
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

    /// 将配置保存到指定路径（序列化为 TOML 格式）
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// 按点分隔的 key 获取配置值（如 "general.default_editor"）
    pub fn get_value(&self, key: &str) -> Option<String> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() != 2 {
            return None;
        }
        let (section, field) = (parts[0], parts[1]);
        match section {
            "general" => match field {
                "default_editor" => Some(self.general.default_editor.clone()),
                "default_priority" => Some(self.general.default_priority.clone()),
                "default_category" => Some(self.general.default_category.clone()),
                "language" => Some(self.general.language.clone()),
                "pager" => Some(self.general.pager.clone()),
                _ => None,
            },
            "display" => match field {
                "color" => Some(self.display.color.to_string()),
                "date_format" => Some(self.display.date_format.clone()),
                "title_max_width" => Some(self.display.title_max_width.to_string()),
                _ => None,
            },
            "storage" => match field {
                "notes_dir" => Some(self.storage.notes_dir.to_string_lossy().to_string()),
                "index_file" => Some(self.storage.index_file.to_string_lossy().to_string()),
                _ => None,
            },
            "search" => match field {
                "default_mode" => Some(self.search.default_mode.clone()),
                "case_sensitive" => Some(self.search.case_sensitive.to_string()),
                "max_results" => Some(self.search.max_results.to_string()),
                _ => None,
            },
            "theme" => match field {
                "title" => Some(self.theme.title.clone()),
                "id" => Some(self.theme.id.clone()),
                "tag" => Some(self.theme.tag.clone()),
                "category" => Some(self.theme.category.clone()),
                "priority_low" => Some(self.theme.priority_low.clone()),
                "priority_normal" => Some(self.theme.priority_normal.clone()),
                "priority_high" => Some(self.theme.priority_high.clone()),
                "priority_urgent" => Some(self.theme.priority_urgent.clone()),
                "separator" => Some(self.theme.separator.clone()),
                "date" => Some(self.theme.date.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    /// 按点分隔的 key 设置配置值，返回是否成功
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(), String> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() != 2 {
            return Err(format!("无效的配置键 '{}', 格式应为 'section.field'", key));
        }
        let (section, field) = (parts[0], parts[1]);
        match section {
            "general" => self.set_general(field, value),
            "display" => self.set_display(field, value),
            "storage" => self.set_storage(field, value),
            "search" => self.set_search(field, value),
            "theme" => self.set_theme(field, value),
            _ => Err(format!("未知的配置段 '{}'", section)),
        }
    }

    fn set_general(&mut self, field: &str, value: &str) -> Result<(), String> {
        match field {
            "default_editor" => { self.general.default_editor = value.to_string(); Ok(()) }
            "default_priority" => {
                Self::validate_priority(value)?;
                self.general.default_priority = value.to_string();
                Ok(())
            }
            "default_category" => { self.general.default_category = value.to_string(); Ok(()) }
            "language" => { self.general.language = value.to_string(); Ok(()) }
            "pager" => { self.general.pager = value.to_string(); Ok(()) }
            _ => Err(format!("未知的配置项 'general.{}'", field)),
        }
    }

    fn set_display(&mut self, field: &str, value: &str) -> Result<(), String> {
        match field {
            "color" => { self.display.color = value.parse::<bool>().map_err(|_| format!("'{}' 不是有效的布尔值", value))?; Ok(()) }
            "date_format" => { self.display.date_format = value.to_string(); Ok(()) }
            "title_max_width" => { self.display.title_max_width = value.parse::<usize>().map_err(|_| format!("'{}' 不是有效的正整数", value))?; Ok(()) }
            _ => Err(format!("未知的配置项 'display.{}'", field)),
        }
    }

    fn set_storage(&mut self, field: &str, value: &str) -> Result<(), String> {
        match field {
            "notes_dir" => { self.storage.notes_dir = PathBuf::from(value); Ok(()) }
            "index_file" => { self.storage.index_file = PathBuf::from(value); Ok(()) }
            _ => Err(format!("未知的配置项 'storage.{}'", field)),
        }
    }

    fn set_search(&mut self, field: &str, value: &str) -> Result<(), String> {
        match field {
            "default_mode" => {
                Self::validate_choice(value, &["plain", "regex", "fuzzy"])?;
                self.search.default_mode = value.to_string();
                Ok(())
            }
            "case_sensitive" => { self.search.case_sensitive = value.parse::<bool>().map_err(|_| format!("'{}' 不是有效的布尔值", value))?; Ok(()) }
            "max_results" => { self.search.max_results = value.parse::<usize>().map_err(|_| format!("'{}' 不是有效的正整数", value))?; Ok(()) }
            _ => Err(format!("未知的配置项 'search.{}'", field)),
        }
    }

    fn set_theme(&mut self, field: &str, value: &str) -> Result<(), String> {
        match field {
            "title" => { self.theme.title = value.to_string(); Ok(()) }
            "id" => { self.theme.id = value.to_string(); Ok(()) }
            "tag" => { self.theme.tag = value.to_string(); Ok(()) }
            "category" => { self.theme.category = value.to_string(); Ok(()) }
            "priority_low" => { self.theme.priority_low = value.to_string(); Ok(()) }
            "priority_normal" => { self.theme.priority_normal = value.to_string(); Ok(()) }
            "priority_high" => { self.theme.priority_high = value.to_string(); Ok(()) }
            "priority_urgent" => { self.theme.priority_urgent = value.to_string(); Ok(()) }
            "separator" => { self.theme.separator = value.to_string(); Ok(()) }
            "date" => { self.theme.date = value.to_string(); Ok(()) }
            _ => Err(format!("未知的配置项 'theme.{}'", field)),
        }
    }

    fn validate_priority(value: &str) -> Result<(), String> {
        Self::validate_choice(value, &["low", "normal", "high", "urgent"])
    }

    fn validate_choice(value: &str, choices: &[&str]) -> Result<(), String> {
        if choices.contains(&value) {
            Ok(())
        } else {
            Err(format!("'{}' 不是有效值，可选: {}", value, choices.join(", ")))
        }
    }

    /// 返回所有可配置的 key 及其当前值
    pub fn all_entries(&self) -> Vec<(&'static str, String)> {
        let mut entries = Vec::new();
        // general
        entries.push(("general.default_editor", self.general.default_editor.clone()));
        entries.push(("general.default_priority", self.general.default_priority.clone()));
        entries.push(("general.default_category", self.general.default_category.clone()));
        entries.push(("general.language", self.general.language.clone()));
        entries.push(("general.pager", self.general.pager.clone()));
        // display
        entries.push(("display.color", self.display.color.to_string()));
        entries.push(("display.date_format", self.display.date_format.clone()));
        entries.push(("display.title_max_width", self.display.title_max_width.to_string()));
        // storage
        entries.push(("storage.notes_dir", self.storage.notes_dir.to_string_lossy().to_string()));
        entries.push(("storage.index_file", self.storage.index_file.to_string_lossy().to_string()));
        // search
        entries.push(("search.default_mode", self.search.default_mode.clone()));
        entries.push(("search.case_sensitive", self.search.case_sensitive.to_string()));
        entries.push(("search.max_results", self.search.max_results.to_string()));
        // theme
        entries.push(("theme.title", self.theme.title.clone()));
        entries.push(("theme.id", self.theme.id.clone()));
        entries.push(("theme.tag", self.theme.tag.clone()));
        entries.push(("theme.category", self.theme.category.clone()));
        entries.push(("theme.priority_low", self.theme.priority_low.clone()));
        entries.push(("theme.priority_normal", self.theme.priority_normal.clone()));
        entries.push(("theme.priority_high", self.theme.priority_high.clone()));
        entries.push(("theme.priority_urgent", self.theme.priority_urgent.clone()));
        entries.push(("theme.separator", self.theme.separator.clone()));
        entries.push(("theme.date", self.theme.date.clone()));
        entries
    }
}