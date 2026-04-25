use std::path::PathBuf;
use std::fs;
use chrono::Datelike;
use dialoguer::{MultiSelect, Select};
use super::super::model::NoteIndexModel;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn handle(
    format: &Option<String>,
    path: &Option<PathBuf>,
    id: &Option<Vec<String>>,
    all: bool,
    category: &Option<String>,
    tag: &Option<Vec<String>>,
    date: &Option<String>,
    storage: &DataBaseStorage,
    output: &Output,
) {
    let notes = collect_notes(id, all, category, tag, date, storage, output);
    if notes.is_empty() {
        output.warn("没有找到匹配的笔记");
        return;
    }

    let format = match format {
        Some(f) => f.clone(),
        None => {
            let formats = ["json", "markdown", "txt", "csv"];
            let sel = match Select::new()
                .with_prompt(format!("选择导出格式 (共 {} 条笔记)", notes.len()))
                .items(&formats)
                .interact()
            {
                Ok(s) => s,
                Err(_) => { output.error("已取消"); return; }
            };
            formats[sel].to_string()
        }
    };

    let dir = match path {
        Some(p) => p.clone(),
        None => PathBuf::from("export"),
    };

    match format.as_str() {
        "txt" => export_txt(&notes, &dir, output),
        "markdown" => export_markdown(&notes, &dir, output),
        "json" => export_json(&notes, &dir, output),
        "csv" => export_csv(&notes, &dir, output),
        _ => output.error(format!("不支持的格式 '{}'", format)),
    }
}

// ─── 数据收集 ───

struct ExportNote {
    index: NoteIndexModel,
    content: String,
}

fn collect_notes(
    id: &Option<Vec<String>>,
    all: bool,
    category: &Option<String>,
    tag: &Option<Vec<String>>,
    date: &Option<String>,
    storage: &DataBaseStorage,
    output: &Output,
) -> Vec<ExportNote> {
    let indexes = storage.list_notes();

    // 无筛选条件时交互选择
    let selected_ids: Option<Vec<u32>> = if id.is_none() && !all && category.is_none() && tag.is_none() && date.is_none() {
        let items: Vec<String> = indexes.iter()
            .map(|n| format!("[{}] {} ({})", n.id, n.title, n.category.name))
            .collect();
        if items.is_empty() {
            return Vec::new();
        }
        let defaults: Vec<bool> = vec![false; items.len()];
        let selections = match MultiSelect::new()
            .with_prompt("选择要导出的笔记（空格切换，回车确认）")
            .items(&items)
            .defaults(&defaults)
            .interact()
        {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        if selections.is_empty() {
            return Vec::new();
        }
        Some(selections.into_iter().map(|i| indexes[i].id).collect())
    } else {
        None
    };

    let mut result = Vec::new();
    for idx in indexes {
        if let Some(ref ids) = selected_ids {
            if !ids.contains(&idx.id) { continue; }
        } else if let Some(ids) = id {
            if !ids.contains(&idx.id.to_string()) { continue; }
        }
        if let Some(cat) = category {
            if idx.category.name != *cat { continue; }
        }
        if let Some(tags) = tag {
            let names: Vec<&str> = idx.tags.iter().map(|t| t.name.as_str()).collect();
            if !tags.iter().all(|t| names.contains(&t.as_str())) { continue; }
        }
        if let Some(expr) = date {
            if !match_date(idx, expr) { continue; }
        }

        match storage.get_note(idx.id) {
            Some(note) => result.push(ExportNote {
                index: idx.clone(),
                content: note.content,
            }),
            None => output.warn(format!("笔记 {} 读取失败，已跳过", idx.id)),
        }
    }
    result
}

fn match_date(note: &NoteIndexModel, expr: &str) -> bool {
    let d = note.modified.date_naive();
    let today = chrono::Local::now().date_naive();
    match expr {
        "today" => d == today,
        "yesterday" => d == today - chrono::TimeDelta::days(1),
        _ => {
            if let Ok(dt) = chrono::NaiveDate::parse_from_str(expr, "%Y-%m-%d") {
                d == dt
            } else if let Some(days) = expr.strip_prefix("last-").and_then(|s| s.trim_end_matches('d').parse::<i64>().ok()) {
                d >= today - chrono::TimeDelta::days(days) && d <= today
            } else {
                false
            }
        }
    }
}

// ─── TXT ───

fn export_txt(notes: &[ExportNote], dir: &PathBuf, output: &Output) {
    ensure_dir(dir, output);
    let mut count = 0;
    for note in notes {
        let path = dir.join(format!("{}.txt", sanitize(&note.index.title)));
        if let Err(e) = fs::write(&path, &note.content) {
            output.warn(format!("写入失败 - {}", e)); continue;
        }
        count += 1;
    }
    output.success(format!("已导出 {} 条笔记 (txt) 到 {}", count, dir.display()));
}

// ─── Markdown ───

fn export_markdown(notes: &[ExportNote], dir: &PathBuf, output: &Output) {
    ensure_dir(dir, output);
    let mut count = 0;
    for note in notes {
        let idx = &note.index;
        let tags: Vec<&str> = idx.tags.iter().map(|t| t.name.as_str()).collect();
        let pri = format!("{:?}", idx.priority).to_lowercase();

        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", idx.title));
        md.push_str(&format!("> ID: {} | 分类: {} | 优先级: {}\n", idx.id, idx.category.name, pri));
        md.push_str(&format!("> 创建: {} | 修改: {}\n\n",
            idx.created.format("%Y-%m-%d %H:%M"),
            idx.modified.format("%Y-%m-%d %H:%M")));
        if !tags.is_empty() {
            let tag_str: Vec<String> = tags.iter().map(|t| format!("`{}`", t)).collect();
            md.push_str(&format!("标签: {}\n\n", tag_str.join(" ")));
        }
        if !note.content.is_empty() {
            md.push_str(&note.content);
            md.push('\n');
        }

        let path = dir.join(format!("{}.md", sanitize(&idx.title)));
        if let Err(e) = fs::write(&path, &md) {
            output.warn(format!("写入失败 - {}", e)); continue;
        }
        count += 1;
    }
    output.success(format!("已导出 {} 条笔记 (markdown) 到 {}", count, dir.display()));
}

// ─── JSON ───

fn export_json(notes: &[ExportNote], dir: &PathBuf, output: &Output) {
    ensure_dir(dir, output);

    let entries: Vec<serde_json::Value> = notes.iter().map(|note| {
        let idx = &note.index;
        let tags: Vec<&str> = idx.tags.iter().map(|t| t.name.as_str()).collect();
        serde_json::json!({
            "id": idx.id,
            "title": idx.title,
            "category": idx.category.name,
            "tags": tags,
            "priority": format!("{:?}", idx.priority).to_lowercase(),
            "created": idx.created.format("%Y-%m-%d %H:%M").to_string(),
            "modified": idx.modified.format("%Y-%m-%d %H:%M").to_string(),
            "content": note.content,
        })
    }).collect();

    let json = serde_json::to_string_pretty(&entries).unwrap();
    let path = dir.join("notes.json");
    if let Err(e) = fs::write(&path, &json) {
        output.error(format!("写入失败 - {}", e));
        return;
    }
    output.success(format!("已导出 {} 条笔记 (json) 到 {}", notes.len(), path.display()));
}

// ─── CSV ───

fn export_csv(notes: &[ExportNote], dir: &PathBuf, output: &Output) {
    ensure_dir(dir, output);

    let mut csv = String::from("ID,标题,分类,标签,优先级,创建时间,修改时间,内容\n");
    for note in notes {
        let idx = &note.index;
        let tags = idx.tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join("|");
        let pri = format!("{:?}", idx.priority).to_lowercase();
        let content = note.content.replace('"', "\"\"").replace('\n', "\\n");
        csv.push_str(&format!(
            "{},\"{}\",\"{}\",\"{}\",{},{},{},\"{}\"\n",
            idx.id,
            idx.title.replace('"', "\"\""),
            idx.category.name.replace('"', "\"\""),
            tags,
            pri,
            idx.created.format("%Y-%m-%d %H:%M"),
            idx.modified.format("%Y-%m-%d %H:%M"),
            content,
        ));
    }

    let path = dir.join("notes.csv");
    if let Err(e) = fs::write(&path, &csv) {
        output.error(format!("写入失败 - {}", e));
        return;
    }
    output.success(format!("已导出 {} 条笔记 (csv) 到 {}", notes.len(), path.display()));
}

// ─── 工具函数 ───

fn ensure_dir(dir: &PathBuf, output: &Output) {
    if let Err(e) = fs::create_dir_all(dir) {
        output.error(format!("创建目录失败 - {}", e));
    }
}

fn sanitize(title: &str) -> String {
    let s: String = title.chars().map(|c| match c {
        '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
        _ => c,
    }).collect();
    let t = s.trim_matches(|c: char| c == '.' || c == ' ');
    if t.is_empty() { "untitled".to_string() } else { t.to_string() }
}
