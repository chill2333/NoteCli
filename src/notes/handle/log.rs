use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use chrono::Local;

pub fn handle(week: bool, date: &Option<String>, storage: &DataBaseStorage, output: &Output) {
    let notes = storage.list_notes();
    let today = Local::now().date_naive();

    let filtered: Vec<_> = if week {
        let start = today - chrono::TimeDelta::days(chrono::Datelike::weekday(&today).num_days_from_monday() as i64);
        notes.iter().filter(|n| n.modified.date_naive() >= start).collect()
    } else if let Some(date_expr) = date {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(date_expr, "%Y-%m-%d") {
            notes.iter().filter(|n| n.modified.date_naive() == d).collect()
        } else {
            output.error("无效日期格式，请使用 YYYY-MM-DD");
            return;
        }
    } else {
        notes.iter().filter(|n| n.modified.date_naive() == today).collect()
    };

    if filtered.is_empty() {
        output.empty("该时间段没有笔记");
        return;
    }

    let mut table = output.create_table();
    output.set_headers(&mut table, &["ID", "标题", "分类", "修改时间"]);

    for n in &filtered {
        table.add_row(vec![
            comfy_table::Cell::new(n.id),
            comfy_table::Cell::new(&n.title),
            comfy_table::Cell::new(&n.category.name),
            comfy_table::Cell::new(n.modified.format("%Y-%m-%d %H:%M").to_string()),
        ]);
    }

    output.print_table(&table);
}
