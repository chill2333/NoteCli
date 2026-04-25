use std::path::PathBuf;
use std::fs;
use chrono::Local;
use dialoguer::Select; // keep for future use
use super::super::model::{NoteModel, NoteIndexModel, Priority};
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;

pub fn handle(
    path: &Option<PathBuf>,
    category: &Option<String>,
    tags: &Option<Vec<String>>,
    storage: &mut DataBaseStorage,
    output: &Output,
) {
    let path = match path {
        Some(p) => p.clone(),
        None => match input::prompt_text("请输入导入文件路径") {
            Some(p) => PathBuf::from(p),
            None => { output.error("已取消"); return; }
        }
    };

    if !path.exists() {
        output.error(format!("文件 '{}' 不存在", path.display()));
        return;
    }

    let raw = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            output.error(format!("读取文件失败 - {}", e));
            return;
        }
    };

    if raw.trim().is_empty() {
        output.error("文件内容为空");
        return;
    }

    let count = match detect_format(&raw) {
        ImportFormat::Json => import_json(&raw, category, tags, storage, output),
        ImportFormat::Markdown => import_markdown(&raw, &path, category, tags, storage, output),
        ImportFormat::PlainText => import_plaintext(&raw, &path, category, tags, storage, output),
    };

    if count > 0 {
        if let Err(e) = storage.save_index() {
            output.error(format!("保存索引失败 - {}", e));
            return;
        }
        output.success(format!("已从 {} 导入 {} 条笔记", path.display(), count));
    }
}

enum ImportFormat {
    Json,
    Markdown,
    PlainText,
}

fn detect_format(raw: &str) -> ImportFormat {
    let trimmed = raw.trim();
    if trimmed.starts_with('[') || trimmed.starts_with('{') {
        ImportFormat::Json
    } else if trimmed.starts_with('#') || trimmed.contains("\n# ") {
        ImportFormat::Markdown
    } else {
        ImportFormat::PlainText
    }
}

// ─── JSON 导入 ───

fn import_json(
    raw: &str,
    default_category: &Option<String>,
    default_tags: &Option<Vec<String>>,
    storage: &mut DataBaseStorage,
    output: &Output,
) -> usize {
    let entries: Vec<serde_json::Value> = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(e) => {
            output.error(format!("JSON 解析失败 - {}", e));
            return 0;
        }
    };

    if entries.is_empty() {
        output.empty("JSON 中没有笔记数据");
        return 0;
    }

    let mut count = 0;
    for entry in &entries {
        let title = entry["title"].as_str().unwrap_or("未命名").to_string();
        let content = entry["content"].as_str().unwrap_or("").to_string();
        let cat_name = entry["category"].as_str()
            .or_else(|| default_category.as_deref())
            .unwrap_or("default");
        let priority = parse_priority_str(entry["priority"].as_str().unwrap_or("normal"));

        let tags: Vec<String> = entry["tags"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let tag_models: Vec<_> = if tags.is_empty() {
            default_tags.as_ref()
                .map(|t| t.iter().map(|name| storage.resolve_tag(name)).collect())
                .unwrap_or_default()
        } else {
            tags.iter().map(|name| storage.resolve_tag(name)).collect()
        };

        let id = storage.next_note_id();
        let now = Local::now();
        let note = NoteModel {
            index: NoteIndexModel {
                id,
                title: resolve_duplicate_title(&title, storage),
                category: storage.resolve_category(cat_name),
                tags: tag_models,
                priority,
                created: now,
                modified: now,
            },
            content,
        };

        if let Err(e) = storage.write_note_file(&note) {
            output.warn(format!(" '{}' 写入失败 - {}", title, e));
            continue;
        }
        storage.add_note(note);
        count += 1;
    }
    count
}

// ─── Markdown 导入 ───

fn import_markdown(
    raw: &str,
    path: &PathBuf,
    default_category: &Option<String>,
    default_tags: &Option<Vec<String>>,
    storage: &mut DataBaseStorage,
    output: &Output,
) -> usize {
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("imported");

    // 按二级标题分割多笔记，若无则视为单笔记
    let sections: Vec<&str> = if raw.contains("\n## ") || raw.starts_with("## ") {
        raw.split("\n## ").collect()
    } else {
        vec![raw]
    };

    let mut count = 0;
    for (i, section) in sections.iter().enumerate() {
        let text = section.trim();
        if text.is_empty() { continue; }

        let (title, content) = extract_title_and_content(text, i, sections.len(), file_stem);

        let id = storage.next_note_id();
        let cat = storage.resolve_category(default_category.as_deref().unwrap_or("default"));
        let tag_models: Vec<_> = default_tags
            .as_ref()
            .map(|t| t.iter().map(|name| storage.resolve_tag(name)).collect())
            .unwrap_or_default();
        let now = Local::now();

        let note = NoteModel {
            index: NoteIndexModel {
                id,
                title: resolve_duplicate_title(&title, storage),
                category: cat,
                tags: tag_models,
                priority: Priority::Normal,
                created: now,
                modified: now,
            },
            content,
        };

        if let Err(e) = storage.write_note_file(&note) {
            output.warn(format!("第 {} 段写入失败 - {}", i + 1, e));
            continue;
        }
        storage.add_note(note);
        count += 1;
    }
    count
}

// ─── 纯文本导入 ───

fn import_plaintext(
    raw: &str,
    path: &PathBuf,
    default_category: &Option<String>,
    default_tags: &Option<Vec<String>>,
    storage: &mut DataBaseStorage,
    output: &Output,
) -> usize {
    let sections: Vec<&str> = raw.split("\n---\n").collect();
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("imported");

    let mut count = 0;
    for (i, section) in sections.iter().enumerate() {
        let text = section.trim();
        if text.is_empty() { continue; }

        let (title, content) = extract_title_and_content(text, i, sections.len(), file_stem);

        let id = storage.next_note_id();
        let cat = storage.resolve_category(default_category.as_deref().unwrap_or("default"));
        let tag_models: Vec<_> = default_tags
            .as_ref()
            .map(|t| t.iter().map(|name| storage.resolve_tag(name)).collect())
            .unwrap_or_default();
        let now = Local::now();

        let note = NoteModel {
            index: NoteIndexModel {
                id,
                title: resolve_duplicate_title(&title, storage),
                category: cat,
                tags: tag_models,
                priority: Priority::Normal,
                created: now,
                modified: now,
            },
            content,
        };

        if let Err(e) = storage.write_note_file(&note) {
            output.warn(format!("第 {} 段写入失败 - {}", i + 1, e));
            continue;
        }
        storage.add_note(note);
        count += 1;
    }
    count
}

// ─── 工具函数 ───

fn extract_title_and_content(text: &str, _index: usize, total: usize, fallback: &str) -> (String, String) {
    // 去掉开头的 # 标记
    let text = text.trim_start_matches('#').trim();

    match text.find('\n') {
        Some(pos) => {
            let title = text[..pos].trim().to_string();
            let content = text[pos + 1..].trim().to_string();
            (title, content)
        }
        None => {
            if total == 1 {
                (fallback.to_string(), text.to_string())
            } else {
                (text.to_string(), String::new())
            }
        }
    }
}

fn parse_priority_str(s: &str) -> Priority {
    match s.to_lowercase().as_str() {
        "low" => Priority::Low,
        "high" => Priority::High,
        "urgent" => Priority::Urgent,
        _ => Priority::Normal,
    }
}

fn resolve_duplicate_title(base: &str, storage: &DataBaseStorage) -> String {
    if !storage.title_exists(base) {
        return base.to_string();
    }
    let mut n = 2;
    loop {
        let candidate = format!("{} ({})", base, n);
        if !storage.title_exists(&candidate) {
            return candidate;
        }
        n += 1;
    }
}
