use dialoguer::{Input, MultiSelect, Select};
use super::storage::DataBaseStorage;
use super::model::{NoteIndexModel, Priority};
use std::io::{self, Write};

pub fn prompt_text(prompt: &str) -> Option<String> {
    Input::<String>::new()
        .with_prompt(prompt)
        .interact_text()
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn prompt_content() -> Option<String> {
    println!("请输入笔记内容（Ctrl+Z + Enter 结束）：");
    let mut lines = Vec::new();
    let stdin = io::stdin();
    loop {
        print!("  > ");
        io::stdout().flush().ok()?;
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
                lines.push(trimmed.to_string());
            }
            Err(_) => return None,
        }
    }
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

pub fn prompt_note_id(storage: &DataBaseStorage) -> Option<u32> {
    let notes: Vec<_> = storage.list_notes().into_iter()
        .filter(|n| !storage.is_archived(n.id))
        .collect();

    if notes.is_empty() {
        return None;
    }

    let items: Vec<String> = notes.iter()
        .map(|n| format!("[{}] {} ({})", n.id, n.title, n.category.name))
        .collect();

    let selection = Select::new()
        .with_prompt("选择笔记")
        .items(&items)
        .interact()
        .ok()?;

    Some(notes[selection].id)
}

pub enum DeleteTarget {
    ById(u32),
    ByTag(Vec<String>),
    ByCategory(String),
    Cancelled,
}

pub fn prompt_delete_target(storage: &DataBaseStorage) -> DeleteTarget {
    let modes = ["按ID删除", "按标签删除", "按分类删除"];
    let choice = match Select::new()
        .with_prompt("选择删除方式")
        .items(&modes)
        .interact()
    {
        Ok(c) => c,
        Err(_) => return DeleteTarget::Cancelled,
    };

    match choice {
        0 => {
            match prompt_note_id(storage) {
                Some(id) => DeleteTarget::ById(id),
                None => DeleteTarget::Cancelled,
            }
        }
        1 => {
            let tags = storage.list_tags();
            if tags.is_empty() {
                return DeleteTarget::Cancelled;
            }
            let items: Vec<String> = tags.iter().map(|t| t.name.clone()).collect();
            let selection = match Select::new()
                .with_prompt("选择标签")
                .items(&items)
                .interact()
            {
                Ok(s) => s,
                Err(_) => return DeleteTarget::Cancelled,
            };
            DeleteTarget::ByTag(vec![items[selection].clone()])
        }
        2 => {
            let cats = storage.list_categories();
            if cats.is_empty() {
                return DeleteTarget::Cancelled;
            }
            let items: Vec<String> = cats.iter().map(|c| c.name.clone()).collect();
            let selection = match Select::new()
                .with_prompt("选择分类")
                .items(&items)
                .interact()
            {
                Ok(s) => s,
                Err(_) => return DeleteTarget::Cancelled,
            };
            DeleteTarget::ByCategory(items[selection].clone())
        }
        _ => DeleteTarget::Cancelled,
    }
}

pub struct EditFields {
    pub title: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<String>,
    pub content: Option<String>,
}

pub fn prompt_edit_fields(note: &NoteIndexModel, storage: &DataBaseStorage) -> Option<EditFields> {
    let fields = ["标题", "分类", "标签", "优先级", "内容"];
    let selections = MultiSelect::new()
        .with_prompt(format!(
            "选择要编辑的字段 (当前: {} / {} / {} / {:?})",
            note.title,
            note.category.name,
            note.tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", "),
            note.priority,
        ))
        .items(&fields)
        .interact()
        .ok()?;

    if selections.is_empty() {
        return None;
    }

    let mut result = EditFields {
        title: None,
        category: None,
        tags: None,
        priority: None,
        content: None,
    };

    for &idx in &selections {
        match idx {
            0 => {
                result.title = Input::<String>::new()
                    .with_prompt("新标题")
                    .default(note.title.clone())
                    .interact_text()
                    .ok();
            }
            1 => {
                let cats = storage.list_categories();
                let mut cat_names: Vec<String> = cats.iter().map(|c| c.name.clone()).collect();
                let new_cat_label = "+ 新建分类";
                cat_names.push(new_cat_label.to_string());
                let default_pos = cat_names.iter().position(|c| *c == note.category.name).unwrap_or(0);
                let selection = match Select::new()
                    .with_prompt("选择分类")
                    .items(&cat_names)
                    .default(default_pos)
                    .interact()
                {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                if cat_names[selection] == new_cat_label {
                    result.category = Input::<String>::new()
                        .with_prompt("输入新分类名称")
                        .interact_text()
                        .ok();
                } else {
                    result.category = Some(cat_names[selection].clone());
                }
            }
            2 => {
                let all_tags = storage.list_tags();
                let tag_names: Vec<&str> = all_tags.iter().map(|t| t.name.as_str()).collect();

                if tag_names.is_empty() {
                    let new_tag = Input::<String>::new()
                        .with_prompt("输入新标签")
                        .interact_text()
                        .ok();
                    if let Some(tag) = new_tag {
                        result.tags = Some(vec![tag]);
                    }
                } else {
                    let defaults: Vec<bool> = tag_names.iter()
                        .map(|name| note.tags.iter().any(|t| t.name == *name))
                        .collect();
                    let selections = match MultiSelect::new()
                        .with_prompt("选择标签（空格切换，回车确认）")
                        .items(&tag_names)
                        .defaults(&defaults)
                        .interact()
                    {
                        Ok(s) => s,
                        Err(_) => continue,
                    };
                    let mut selected_tags: Vec<String> = selections.into_iter()
                        .map(|i| tag_names[i].to_string())
                        .collect();
                    if let Some(new_tag) = Input::<String>::new()
                        .with_prompt("添加新标签（留空跳过）")
                        .allow_empty(true)
                        .interact_text()
                        .ok()
                        .filter(|s| !s.trim().is_empty())
                    {
                        selected_tags.push(new_tag);
                    }
                    result.tags = Some(selected_tags);
                }
            }
            3 => {
                let priorities = ["low", "normal", "high", "urgent"];
                let default_pos = match note.priority {
                    Priority::Low => 0,
                    Priority::Normal => 1,
                    Priority::High => 2,
                    Priority::Urgent => 3,
                };
                let selection = match Select::new()
                    .with_prompt("选择优先级")
                    .items(&priorities)
                    .default(default_pos)
                    .interact()
                {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                result.priority = Some(priorities[selection].to_string());
            }
            4 => {
                println!("当前内容：");
                let note_content = storage.get_note(note.id).map(|n| n.content.clone()).unwrap_or_default();
                for line in note_content.lines() {
                    println!("    {}", line);
                }
                result.content = prompt_content();
            }
            _ => {}
        }
    }

    Some(result)
}
