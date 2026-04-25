use comfy_table::{Cell, Attribute};
use super::super::storage::DataBaseStorage;
use super::super::model::NoteIndexModel;
use super::super::output::Output;

pub fn handle(
    sort: &Option<String>,
    limit: &Option<u32>,
    offset: &Option<u32>,
    category: &Option<String>,
    tag: &Option<Vec<String>>,
    priority: &Option<String>,
    date: &Option<String>,
    hastag: bool,
    notag: bool,
    storage: &DataBaseStorage,
    output: &Output,
) {
    let notes: Vec<&NoteIndexModel> = storage.list_notes();

    let mut filtered: Vec<&NoteIndexModel> = notes;

    // 默认隐藏已归档笔记
    filtered.retain(|n| !storage.is_archived(n.id));

    if let Some(cat) = category {
        filtered.retain(|n| n.category.name == *cat);
    }
    if let Some(tags) = tag {
        filtered.retain(|n| {
            let note_tag_names: Vec<&str> = n.tags.iter().map(|t| t.name.as_str()).collect();
            tags.iter().all(|t| note_tag_names.contains(&t.as_str()))
        });
    }
    if let Some(p) = priority {
        filtered.retain(|n| format!("{:?}", n.priority).to_lowercase() == p.to_lowercase());
    }
    if hastag {
        filtered.retain(|n| !n.tags.is_empty());
    }
    if notag {
        filtered.retain(|n| n.tags.is_empty());
    }
    if let Some(date_expr) = date {
        filtered.retain(|n| match_date(n, date_expr));
    }

    match sort.as_deref().unwrap_or("modified") {
        "created" => filtered.sort_by_key(|n| n.created),
        "title" => filtered.sort_by(|a, b| a.title.cmp(&b.title)),
        "priority" => filtered.sort_by_key(|n| n.priority.clone()),
        _ => filtered.sort_by_key(|n| n.modified),
    }

    let (pinned, mut rest): (Vec<_>, Vec<_>) = filtered.into_iter()
        .partition(|n| storage.is_pinned(n.id));
    let mut notes = pinned;
    notes.append(&mut rest);

    let skip = offset.unwrap_or(0) as usize;
    let take = limit.map(|l| l as usize);
    if skip < notes.len() {
        notes = notes.into_iter().skip(skip).collect();
    } else {
        notes.clear();
    }
    if let Some(t) = take {
        notes.truncate(t);
    }

    if notes.is_empty() {
        output.empty("没有找到笔记");
        return;
    }

    let mut table = output.create_table();
    output.set_headers(&mut table, &["ID", "标题", "分类", "优先级", "修改时间", "标签", "状态"]);

    for n in &notes {
        let tags: String = n.tags.iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let pri_str = format!("{:?}", n.priority).to_lowercase();

        let pinned = storage.is_pinned(n.id);
        let done = storage.is_done(n.id);
        let mut status_parts = Vec::new();
        if pinned { status_parts.push("📌"); }
        if done { status_parts.push("[DONE]"); }
        let status_text = status_parts.join(" ");

        table.add_row(vec![
            output.cell_id(n.id),
            output.cell_title(&n.title),
            output.cell_category(&n.category.name),
            output.cell_priority(&pri_str),
            output.cell_date(&n.modified.format("%Y-%m-%d %H:%M").to_string()),
            output.cell_tag(if tags.is_empty() { "-" } else { &tags }),
            Cell::new(status_text).add_attribute(Attribute::Dim),
        ]);
    }

    output.print_table(&table);
    output.line(format!("共 {} 条", notes.len()));
}

fn match_date(note: &NoteIndexModel, expr: &str) -> bool {
    let date = note.modified.date_naive();
    let today = chrono::Local::now().date_naive();
    match expr {
        "today" => date == today,
        "yesterday" => date == today - chrono::TimeDelta::days(1),
        _ => {
            if let Ok(d) = chrono::NaiveDate::parse_from_str(expr, "%Y-%m-%d") {
                date == d
            } else if let Some(days) = expr.strip_prefix("last-").and_then(|s| s.trim_end_matches('d').parse::<i64>().ok()) {
                date >= today - chrono::TimeDelta::days(days) && date <= today
            } else {
                false
            }
        }
    }
}
