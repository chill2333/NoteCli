use colored::Colorize;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;

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
    let old = match old_name {
        Some(n) => n,
        None => {
            output.error("请指定当前分类名称");
            return;
        }
    };
    let new = match new_name {
        Some(n) => n,
        None => {
            output.error("请指定新分类名称");
            return;
        }
    };

    if !storage.list_categories().iter().any(|c| c.name == *old) {
        output.error(format!("分类 '{}' 不存在", old));
        return;
    }

    storage.rename_category(old, new);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("分类 '{}' 已重命名为 '{}'", old, new));
}

pub fn delete(name: &str, force: bool, keep: bool, storage: &mut DataBaseStorage, output: &Output) {
    if !storage.list_categories().iter().any(|c| c.name == name) {
        output.error(format!("分类 '{}' 不存在", name));
        return;
    }

    let count = storage.list_notes().iter().filter(|n| n.category.name == name).count();

    if !force {
        if keep {
            output.info(format!("确定要删除分类 '{}' 吗？该分类下 {} 条笔记的 category 字段将被清空。使用 -f 确认", name, count));
        } else {
            output.info(format!("确定要删除分类 '{}' 及其 {} 条笔记吗？使用 -f 确认", name, count));
        }
        return;
    }

    if keep {
        storage.delete_category_keep_notes(name);
        output.success(format!("分类 '{}' 已删除，{} 条笔记已保留（category 已重置为 default）", name, count));
    } else {
        storage.delete_category_with_notes(name);
        output.success(format!("分类 '{}' 及其 {} 条笔记已删除", name, count));
    }
}
