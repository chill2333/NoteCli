use colored::Colorize;
use super::super::storage::DataBaseStorage;
use super::super::model::Priority;
use super::super::output::Output;

pub fn handle(storage: &DataBaseStorage, output: &Output) {
    let notes = storage.list_notes();
    let total = notes.len();

    if total == 0 {
        output.empty("暂无笔记");
        return;
    }

    let pinned = notes.iter().filter(|n| storage.is_pinned(n.id)).count();
    let archived = storage.note_status_ref().archived_notes.len();

    let by_priority = |p: Priority| notes.iter().filter(|n| n.priority == p).count();

    output.blank();
    output.line(format!("  {}", "笔记统计".cyan().bold()));
    output.line(format!("  {}", "─".repeat(30).bright_black()));

    let mut table = output.create_table();
    output.set_headers(&mut table, &["指标", "数量"]);

    table.add_row(vec![comfy_table::Cell::new("总数"), comfy_table::Cell::new(total)]);
    table.add_row(vec![comfy_table::Cell::new("已置顶"), comfy_table::Cell::new(pinned)]);
    table.add_row(vec![comfy_table::Cell::new("已归档"), comfy_table::Cell::new(archived)]);
    table.add_row(vec![comfy_table::Cell::new("低优先级"), comfy_table::Cell::new(by_priority(Priority::Low))]);
    table.add_row(vec![comfy_table::Cell::new("普通优先级"), comfy_table::Cell::new(by_priority(Priority::Normal))]);
    table.add_row(vec![comfy_table::Cell::new("高优先级"), comfy_table::Cell::new(by_priority(Priority::High))]);
    table.add_row(vec![comfy_table::Cell::new("紧急"), comfy_table::Cell::new(by_priority(Priority::Urgent))]);

    let cats = storage.list_categories().len();
    let tags = storage.list_tags().len();
    table.add_row(vec![comfy_table::Cell::new("分类数"), comfy_table::Cell::new(cats)]);
    table.add_row(vec![comfy_table::Cell::new("标签数"), comfy_table::Cell::new(tags)]);

    output.print_table(&table);
    output.blank();
}
