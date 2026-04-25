use super::super::storage::DataBaseStorage;
use super::super::output::Output;

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
    let old = match old_name {
        Some(n) => n,
        None => {
            output.error("请指定当前标签名称");
            return;
        }
    };
    let new = match new_name {
        Some(n) => n,
        None => {
            output.error("请指定新标签名称");
            return;
        }
    };

    if !storage.list_tags().iter().any(|t| t.name == *old) {
        output.error(format!("标签 '{}' 不存在", old));
        return;
    }

    storage.rename_tag(old, new);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("标签 '{}' 已重命名为 '{}'", old, new));
}

pub fn delete(name: &str, force: bool, storage: &mut DataBaseStorage, output: &Output) {
    if !storage.list_tags().iter().any(|t| t.name == name) {
        output.error(format!("标签 '{}' 不存在", name));
        return;
    }

    if !force {
        let count = storage.list_notes().iter().filter(|n| n.tags.iter().any(|t| t.name == name)).count();
        output.info(format!("标签 '{}' 被 {} 条笔记使用，使用 -f 确认删除", name, count));
        return;
    }

    storage.delete_tag(name);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("标签 '{}' 已删除", name));
}
