use dialoguer::{Input, MultiSelect, Select};
use super::storage::DataBaseStorage;
use super::model::{NoteIndexModel, Priority};
use std::io::{self, Write};
use colored::Colorize;

fn banner(title: &str) {
    println!();
    println!("  {} {}", ">>".cyan().bold(), title.bold());
    println!("  {}", "─".repeat(40).bright_black());
}

fn section(title: &str) {
    println!("  {} {}", "●".cyan(), title.bold());
}

fn hint(msg: &str) {
    println!("  {} {}", "?".yellow(), msg.bright_black());
}

pub fn prompt_text(prompt: &str) -> Option<String> {
    section(prompt);
    Input::<String>::new()
        .with_prompt("  输入")
        .interact_text()
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn prompt_content() -> Option<String> {
    banner("输入笔记内容");
    hint("逐行输入，Ctrl+Z + Enter (Windows) 或 Ctrl+D (Unix) 结束");
    println!();
    let mut lines = Vec::new();
    let stdin = io::stdin();
    loop {
        print!("  {} ", ">".green().bold());
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

    banner("选择笔记");
    hint("使用 ↑↓ 选择，Enter 确认");

    let items: Vec<String> = notes.iter()
        .map(|n| {
            let pri_icon = match n.priority {
                Priority::Urgent => "!!".red().bold(),
                Priority::High => "!".yellow(),
                Priority::Normal => " ".normal(),
                Priority::Low => " ".dimmed(),
            };
            let tags = if n.tags.is_empty() {
                String::new()
            } else {
                format!(" {}", n.tags.iter().map(|t| format!("#{}", t.name)).collect::<Vec<_>>().join(" ").bright_black())
            };
            format!("{} [{}] {} ({}){}", pri_icon, n.id, n.title, n.category.name, tags)
        })
        .collect();

    let selection = Select::new()
        .with_prompt("  笔记")
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
    banner("删除笔记");

    let modes = [
        "按 ID 删除    选择单条笔记删除",
        "按标签删除    删除包含指定标签的所有笔记",
        "按分类删除    删除指定分类下的所有笔记",
    ];
    hint("选择删除方式");
    let choice = match Select::new()
        .with_prompt("  方式")
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
            let items: Vec<String> = tags.iter()
                .map(|t| {
                    let count = storage.list_notes().iter().filter(|n| n.tags.iter().any(|nt| nt.name == t.name)).count();
                    format!("#{} ({} 条笔记)", t.name, count)
                })
                .collect();
            section("选择标签");
            let selection = match Select::new()
                .with_prompt("  标签")
                .items(&items)
                .interact()
            {
                Ok(s) => s,
                Err(_) => return DeleteTarget::Cancelled,
            };
            DeleteTarget::ByTag(vec![tags[selection].name.clone()])
        }
        2 => {
            let cats = storage.list_categories();
            if cats.is_empty() {
                return DeleteTarget::Cancelled;
            }
            let items: Vec<String> = cats.iter()
                .map(|c| {
                    let count = storage.list_notes().iter().filter(|n| n.category.name == c.name).count();
                    format!("{} ({} 条笔记)", c.name, count)
                })
                .collect();
            section("选择分类");
            let selection = match Select::new()
                .with_prompt("  分类")
                .items(&items)
                .interact()
            {
                Ok(s) => s,
                Err(_) => return DeleteTarget::Cancelled,
            };
            DeleteTarget::ByCategory(cats[selection].name.clone())
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
    banner("编辑笔记");

    let tags_display = if note.tags.is_empty() {
        "无".to_string()
    } else {
        note.tags.iter().map(|t| format!("#{}", t.name)).collect::<Vec<_>>().join(" ")
    };
    let pri_display = format!("{:?}", note.priority).to_lowercase();
    let pri_icon = match note.priority {
        Priority::Urgent => "!!",
        Priority::High => "!",
        _ => " ",
    };

    println!("  {} {} {}", pri_icon, note.title.bold(), format!("[{}]", note.id).bright_black());
    println!("  {} 分类: {}  标签: {}  优先级: {}", "·".bright_black(), note.category.name.cyan(), tags_display, pri_display);
    println!();

    hint("空格切换选择，Enter 确认");

    let fields = [
        "标题    修改笔记标题",
        "分类    移动到其他分类",
        "标签    更新标签（可多选）",
        "优先级  调整优先级",
        "内容    替换笔记内容",
    ];
    let selections = MultiSelect::new()
        .with_prompt("  选择要编辑的字段")
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
                section("修改标题");
                result.title = Input::<String>::new()
                    .with_prompt(format!("  新标题 {}", format!("(当前: {})", note.title).bright_black()))
                    .default(note.title.clone())
                    .interact_text()
                    .ok();
            }
            1 => {
                section("修改分类");
                let cats = storage.list_categories();
                let mut cat_names: Vec<String> = cats.iter().map(|c| c.name.clone()).collect();
                let new_cat_label = "+ 新建分类";
                cat_names.push(new_cat_label.to_string());
                let default_pos = cat_names.iter().position(|c| *c == note.category.name).unwrap_or(0);
                let selection = match Select::new()
                    .with_prompt("  选择分类")
                    .items(&cat_names)
                    .default(default_pos)
                    .interact()
                {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                if cat_names[selection] == new_cat_label {
                    result.category = Input::<String>::new()
                        .with_prompt("  输入新分类名称")
                        .interact_text()
                        .ok();
                } else {
                    result.category = Some(cat_names[selection].clone());
                }
            }
            2 => {
                section("修改标签");
                let all_tags = storage.list_tags();
                let tag_names: Vec<&str> = all_tags.iter().map(|t| t.name.as_str()).collect();

                if tag_names.is_empty() {
                    result.tags = Input::<String>::new()
                        .with_prompt("  输入新标签")
                        .interact_text()
                        .ok()
                        .map(|t| vec![t]);
                } else {
                    hint("空格切换，Enter 确认");
                    let defaults: Vec<bool> = tag_names.iter()
                        .map(|name| note.tags.iter().any(|t| t.name == *name))
                        .collect();
                    let selections = match MultiSelect::new()
                        .with_prompt("  选择标签")
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
                        .with_prompt("  添加新标签（留空跳过）")
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
                section("修改优先级");
                let priorities = [
                    "low     低优先级",
                    "normal  普通优先级",
                    "high    高优先级",
                    "urgent  紧急",
                ];
                let default_pos = match note.priority {
                    Priority::Low => 0,
                    Priority::Normal => 1,
                    Priority::High => 2,
                    Priority::Urgent => 3,
                };
                let selection = match Select::new()
                    .with_prompt("  选择优先级")
                    .items(&priorities)
                    .default(default_pos)
                    .interact()
                {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                result.priority = Some(["low", "normal", "high", "urgent"][selection].to_string());
            }
            4 => {
                section("修改内容");
                println!("  {} {}", "当前内容:".bright_black(), "─".repeat(30).bright_black());
                let note_content = storage.get_note(note.id).map(|n| n.content.clone()).unwrap_or_default();
                for line in note_content.lines() {
                    println!("    {}", line);
                }
                println!("  {}", "─".repeat(40).bright_black());
                result.content = prompt_content();
            }
            _ => {}
        }
    }

    Some(result)
}
