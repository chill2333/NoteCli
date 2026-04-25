use colored::Colorize;
use dialoguer::Select;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;

pub fn list(storage: &DataBaseStorage, output: &Output) {
    let categories = storage.list_categories();
    if categories.is_empty() {
        output.empty("没有分类");
        return;
    }

    let mut table = output.create_table();
    output.set_headers(&mut table, &["ID", "名称", "笔记数"]);

    for cat in categories {
        let count = storage.list_notes().iter().filter(|n| n.category.name == cat.name).count();
        table.add_row(vec![
            comfy_table::Cell::new(cat.id),
            comfy_table::Cell::new(&cat.name),
            comfy_table::Cell::new(count),
        ]);
    }

    output.print_table(&table);
}

pub fn tree(storage: &DataBaseStorage, output: &Output) {
    let categories = storage.list_categories();
    if categories.is_empty() {
        output.empty("没有分类");
        return;
    }

    for cat in categories {
        let count = storage.list_notes().iter().filter(|n| n.category.name == cat.name).count();
        output.line(format!("  {} ({} 条笔记)", cat.name.cyan(), count));
    }
}

pub fn rename(old_name: &Option<String>, new_name: &Option<String>, storage: &mut DataBaseStorage, output: &Output) {
    let cats = storage.list_categories();
    if cats.is_empty() {
        output.empty("没有分类");
        return;
    }

    let cat_names: Vec<&str> = cats.iter().map(|c| c.name.as_str()).collect();

    let old = match old_name {
        Some(n) => n.clone(),
        None => {
            let selection = match Select::new()
                .with_prompt("选择要重命名的分类")
                .items(&cat_names)
                .interact()
            {
                Ok(s) => s,
                Err(_) => { output.error("已取消"); return; }
            };
            cat_names[selection].to_string()
        }
    };

    if !cat_names.contains(&old.as_str()) {
        output.error(format!("分类 '{}' 不存在", old));
        return;
    }

    let new = match new_name {
        Some(n) => n.clone(),
        None => match input::prompt_text(&format!("新名称 (当前: {})", old)) {
            Some(n) => n,
            None => { output.error("已取消"); return; }
        }
    };

    storage.rename_category(&old, &new);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("分类 '{}' 已重命名为 '{}'", old, new));
}
