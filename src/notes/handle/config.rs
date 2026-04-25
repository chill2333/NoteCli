use std::path::PathBuf;
use super::super::config::Config;
use super::super::output::Output;
use super::super::input;

pub fn list(output: &Output, config_path: &PathBuf) {
    let cfg = load_or_default(config_path, output);

    let mut table = output.create_table();
    output.set_headers(&mut table, &["配置项", "值"]);

    for (key, value) in cfg.all_entries() {
        table.add_row(vec![
            comfy_table::Cell::new(key),
            comfy_table::Cell::new(value),
        ]);
    }

    output.print_table(&table);
    output.hint(format!("配置文件: {}", config_path.display()));
}

pub fn get(key: &Option<String>, output: &Output, config_path: &PathBuf) {
    let key = match key {
        Some(k) => k.clone(),
        None => match input::prompt_text("配置项名称 (如 general.default_editor)") {
            Some(k) => k,
            None => { output.error("已取消"); return; }
        }
    };

    let cfg = load_or_default(config_path, output);

    match cfg.get_value(&key) {
        Some(value) => {
            output.line(format!("{} = {}", key, value));
        }
        None => {
            output.error(format!("未知的配置项 '{}', 格式为 'section.field'（如 general.default_editor）", key));
        }
    }
}

pub fn set(key: &Option<String>, value: &Option<String>, output: &Output, config_path: &PathBuf) {
    let key = match key {
        Some(k) => k.clone(),
        None => match input::prompt_text("配置项名称 (如 general.default_editor)") {
            Some(k) => k,
            None => { output.error("已取消"); return; }
        }
    };

    let value = match value {
        Some(v) => v.clone(),
        None => match input::prompt_text(&format!("{} 的值", key)) {
            Some(v) => v,
            None => { output.error("已取消"); return; }
        }
    };

    let mut cfg = load_or_default(config_path, output);

    if let Err(e) = cfg.set_value(&key, &value) {
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
