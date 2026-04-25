use super::super::model::{NoteModel, NoteIndexModel, Priority};
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use chrono::Local;

pub fn handle(
    content: &str,
    title: &Option<String>,
    category: &Option<String>,
    tags: &Option<Vec<String>>,
    priority: &Option<String>,
    storage: &mut DataBaseStorage,
    output: &Output,
) {
    if content.trim().is_empty() {
        output.error("笔记内容不能为空");
        return;
    }

    let base_title = title.clone().unwrap_or_else(|| {
        let chars: String = content.chars().take(20).collect();
        if chars.is_empty() { "未命名笔记".to_string() } else { chars }
    });
    let note_title = resolve_duplicate_title(&base_title, storage);
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
            title: note_title,
            category: cat,
            tags: tag_models,
            priority: parse_priority(priority.as_deref().unwrap_or("normal")),
            created: now,
            modified: now,
        },
        content: content.to_string(),
    };

    if let Err(e) = storage.write_note_file(&note) {
        output.error(format!("写入笔记文件失败 - {}", e));
        return;
    }

    let id = note.index.id;
    storage.add_note(note);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("笔记已创建 (ID: {})", id));
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

fn parse_priority(s: &str) -> Priority {
    match s.to_lowercase().as_str() {
        "low" => Priority::Low,
        "high" => Priority::High,
        "urgent" => Priority::Urgent,
        _ => Priority::Normal,
    }
}
