use colored::Colorize;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn handle(id: &Option<u32>, raw: bool, storage: &DataBaseStorage, output: &Output) {
    let id = match id {
        Some(id) => *id,
        None => {
            output.error("请指定笔记ID");
            return;
        }
    };

    match storage.get_note(id) {
        Some(note) => {
            if raw {
                output.line(format!("标题: {}", note.index.title));
                output.line(format!("ID: {}", note.index.id));
                output.line(format!("分类: {}", note.index.category.name));
                output.line(format!("优先级: {:?}", note.index.priority));
                output.line(format!("创建: {}", note.index.created.format("%Y-%m-%d %H:%M")));
                output.line(format!("修改: {}", note.index.modified.format("%Y-%m-%d %H:%M")));
                if !note.index.tags.is_empty() {
                    let tags: Vec<&str> = note.index.tags.iter().map(|t| t.name.as_str()).collect();
                    output.line(format!("标签: {}", tags.join(", ")));
                }
                if !note.content.is_empty() {
                    output.line("---");
                    output.line(&note.content);
                }
                return;
            }

            let theme = output.theme();
            let title = output.styled(&note.index.title, &theme.title).bold();
            let id_str = output.styled(&note.index.id.to_string(), &theme.id);
            let cat = output.styled(&note.index.category.name, &theme.category);
            let pri = format!("{:?}", note.index.priority).to_lowercase();
            let pri = output.styled(&pri, output.priority_style(&pri));
            let created = output.styled(&note.index.created.format("%Y-%m-%d %H:%M").to_string(), &theme.date);
            let modified = output.styled(&note.index.modified.format("%Y-%m-%d %H:%M").to_string(), &theme.date);

            output.blank();
            output.line(format!("  {} {}\n", title, format!("[{}]", id_str)));

            output.line(format!("  分类: {}  优先级: {}", cat, pri));
            output.line(format!("  创建: {}  修改: {}", created, modified));

            if !note.index.tags.is_empty() {
                let tags: Vec<&str> = note.index.tags.iter().map(|t| t.name.as_str()).collect();
                output.line(format!("  标签: {}", output.styled(&tags.join(", "), &theme.tag)));
            }

            if !note.content.is_empty() {
                output.line(format!("  {}", output.styled(&"─".repeat(50), &theme.separator)));
                for line in note.content.lines() {
                    output.line(format!("  {}", line));
                }
            }
            output.blank();
        }
        None => output.error(format!("笔记ID {} 不存在", id)),
    }
}
