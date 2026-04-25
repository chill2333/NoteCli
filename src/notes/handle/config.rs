use std::path::PathBuf;
use super::super::config::Config;
use super::super::output::Output;

/// `note config list` — 列出所有配置项
pub fn list(output: &Output, config_path: &PathBuf) {
    let cfg = load_or_default(config_path, output);

    let mut table = output.create_table();
    output.set_headers(&mut table, &["配置项", "值"]);

    let mut current_section = "";
    for (key, value) in cfg.all_entries() {
        let section = key.split('.').next().unwrap_or("");
        if section != current_section {
            current_section = section;
        }
        table.add_row(vec![
            comfy_table::Cell::new(key),
            comfy_table::Cell::new(value),
        ]);
    }

    output.print_table(&table);
    output.hint(format!("配置文件: {}", config_path.display()));
}

/// `note config get <key>` — 获取指定配置项的值
pub fn get(key: &str, output: &Output, config_path: &PathBuf) {
    let cfg = load_or_default(config_path, output);

    match cfg.get_value(key) {
        Some(value) => {
            output.line(format!("{} = {}", key, value));
        }
        None => {
            output.error(format!("未知的配置项 '{}', 格式为 'section.field'（如 general.default_editor）", key));
        }
    }
}

/// `note config set <key> <value>` — 设置配置项的值并持久化
pub fn set(key: &str, value: &str, output: &Output, config_path: &PathBuf) {
    let mut cfg = load_or_default(config_path, output);

    if let Err(e) = cfg.set_value(key, value) {
        output.error(e);
        return;
    }

    if let Err(e) = cfg.save_to_file(config_path) {
        output.error(format!("保存配置文件失败: {}", e));
        return;
    }

    output.success(format!("{} = {}", key, value));
    output.hint(format!("已保存到 {}", config_path.display()));
}

/// 加载配置文件，不存在时返回默认值
fn load_or_default(path: &PathBuf, output: &Output) -> Config {
    if path.exists() {
        match Config::from_file(Some(path.to_string_lossy().to_string().as_str())) {
            Ok(cfg) => return cfg,
            Err(e) => {
                output.warn(format!("配置文件解析失败，使用默认值: {}", e));
            }
        }
    }
    Config::default()
}
