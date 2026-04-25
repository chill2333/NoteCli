use std::path::PathBuf;
use std::fs;
use chrono::Local;
use super::super::model::{NoteModel, NoteIndexModel, Priority};
use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn handle(
    path: &PathBuf,
    category: &Option<String>,
    tags: &Option<Vec<String>>,
    storage: &mut DataBaseStorage,
    output: &Output,
) {
    if !path.exists() {
        output.error(format!("文件 '{}' 不存在", path.display()));
        return;
    }

    let raw = match fs::read_to_string(path) {
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

    let sections: Vec<&str> = raw.split("\n---\n").collect();
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("imported");

    let mut count = 0;
    for (i, section) in sections.iter().enumerate() {
        let text = section.trim();
        if text.is_empty() {
            continue;
        }

        // 第一行作为标题，其余作为正文
        let (title, content) = match text.find('\n') {
            Some(pos) => (text[..pos].trim().to_string(), text[pos + 1..].trim().to_string()),
            None => {
                // 只有一行：如果只有一个段落，用文件名作标题
                if sections.len() == 1 {
                    (file_stem.to_string(), text.to_string())
                } else {
                    (text.to_string(), String::new())
                }
            }
        };

        let id = storage.next_note_id();
        let cat = storage.resolve_category(category.as_deref().unwrap_or("default"));
        let tag_models: Vec<_> = tags
            .as_ref()
            .map(|t| t.iter().map(|name| storage.resolve_tag(name)).collect())
            .unwrap_or_default();
        let now = Local::now();

        let note = NoteModel {
            index: NoteIndexModel {
                id,
                title,
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

    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("已从 {} 导入 {} 条笔记", path.display(), count));
}
