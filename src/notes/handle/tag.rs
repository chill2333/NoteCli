use dialoguer::Select;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;

pub fn list(storage: &DataBaseStorage, output: &Output) {
    let tags = storage.list_tags();
    if tags.is_empty() {
        output.empty("没有标签");
        return;
    }

    let mut table = output.create_table();
    output.set_headers(&mut table, &["ID", "名称", "笔记数"]);

    for tag in tags {
        let count = storage.list_notes().iter().filter(|n| n.tags.iter().any(|t| t.name == tag.name)).count();
        table.add_row(vec![
            comfy_table::Cell::new(tag.id),
            comfy_table::Cell::new(&tag.name),
            comfy_table::Cell::new(count),
        ]);
    }

    output.print_table(&table);
}

pub fn rename(old_name: &Option<String>, new_name: &Option<String>, storage: &mut DataBaseStorage, output: &Output) {
    let tags = storage.list_tags();
    if tags.is_empty() {
        output.empty("没有标签");
        return;
    }

    let tag_names: Vec<&str> = tags.iter().map(|t| t.name.as_str()).collect();

    let old = match old_name {
        Some(n) => n.clone(),
        None => {
            let selection = match Select::new()
                .with_prompt("选择要重命名的标签")
                .items(&tag_names)
                .interact()
            {
                Ok(s) => s,
                Err(_) => { output.error("已取消"); return; }
            };
            tag_names[selection].to_string()
        }
    };

    if !tag_names.contains(&old.as_str()) {
        output.error(format!("标签 '{}' 不存在", old));
        return;
    }

    let new = match new_name {
        Some(n) => n.clone(),
        None => match input::prompt_text(&format!("新名称 (当前: {})", old)) {
            Some(n) => n,
            None => { output.error("已取消"); return; }
        }
    };

    storage.rename_tag(&old, &new);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("标签 '{}' 已重命名为 '{}'", old, new));
}
