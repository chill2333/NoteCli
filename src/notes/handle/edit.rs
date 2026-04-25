use super::super::model::Priority;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use chrono::Local;

pub fn handle(
    id: u32,
    title: &Option<String>,
    category: &Option<String>,
    tags: &Option<Vec<String>>,
    priority: &Option<String>,
    content: &Option<String>,
    append: &Option<String>,
    storage: &mut DataBaseStorage,
    output: &Output,
) {
    let mut note = match storage.get_note(id) {
        Some(n) => n,
        None => {
            output.error(format!("笔记ID {} 不存在", id));
            return;
        }
    };

    if storage.is_archived(id) {
        output.error(format!("笔记 {} 已归档，无法编辑。请先使用 unarchive 取消归档", id));
        return;
    }

    if let Some(t) = title {
        note.index.title = t.clone();
    }
    if let Some(c) = category {
        note.index.category = storage.resolve_category(c);
    }
    if let Some(t) = tags {
        note.index.tags = t.iter().map(|name| storage.resolve_tag(name)).collect();
    }
    if let Some(p) = priority {
        note.index.priority = parse_priority(p);
    }
    if let Some(c) = content {
        note.content = c.clone();
    } else if let Some(a) = append {
        if note.content.is_empty() {
            note.content = a.clone();
        } else {
            note.content = format!("{}\n{}", note.content, a);
        }
    }

    note.index.modified = Local::now();

    if let Err(e) = storage.update_note(note) {
        output.error(format!("更新笔记失败 - {}", e));
        return;
    }
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("笔记 {} 已更新", id));
}

fn parse_priority(s: &str) -> Priority {
    match s.to_lowercase().as_str() {
        "low" => Priority::Low,
        "high" => Priority::High,
        "urgent" => Priority::Urgent,
        _ => Priority::Normal,
    }
}
