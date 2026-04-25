use super::super::model::Priority;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;
use chrono::Local;

pub fn handle(
    id: Option<u32>,
    title: &Option<String>,
    category: &Option<String>,
    tags: &Option<Vec<String>>,
    priority: &Option<String>,
    content: &Option<String>,
    append: &Option<String>,
    storage: &mut DataBaseStorage,
    output: &Output,
) {
    let id = match id {
        Some(id) => id,
        None => match input::prompt_note_id(storage) {
            Some(id) => id,
            None => { output.error("已取消"); return; }
        }
    };

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

    let (title, category, tags, priority, content) = if title.is_none()
        && category.is_none() && tags.is_none()
        && priority.is_none() && content.is_none() && append.is_none()
    {
        match input::prompt_edit_fields(&note.index, storage) {
            Some(fields) => (fields.title, fields.category, fields.tags, fields.priority, fields.content),
            None => { output.error("已取消"); return; }
        }
    } else {
        (title.clone(), category.clone(), tags.clone(), priority.clone(), content.clone())
    };

    if let Some(t) = title {
        if t != note.index.title {
            let resolved = resolve_duplicate_title(&t, storage);
            if resolved != t {
                output.warn(format!("标题 \"{}\" 已存在，自动重命名为 \"{}\"", t, resolved));
            }
            note.index.title = resolved;
        }
    }
    if let Some(c) = category {
        note.index.category = storage.resolve_category(&c);
    }
    if let Some(t) = tags {
        note.index.tags = t.iter().map(|name| storage.resolve_tag(name)).collect();
    }
    if let Some(p) = priority {
        note.index.priority = parse_priority(&p);
    }
    if let Some(c) = content {
        note.content = c;
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
