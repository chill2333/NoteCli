use dialoguer::Confirm;
use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;

pub fn handle(
    id: Option<u32>,
    tag: &Option<Vec<String>>,
    category: &Option<String>,
    force: bool,
    storage: &mut DataBaseStorage,
    output: &Output,
) {
    let (id, tag, category) = if id.is_none() && tag.is_none() && category.is_none() {
        match input::prompt_delete_target(storage) {
            input::DeleteTarget::ById(id) => (Some(id), None, None),
            input::DeleteTarget::ByTag(tags) => (None, Some(tags), None),
            input::DeleteTarget::ByCategory(cat) => (None, None, Some(cat)),
            input::DeleteTarget::Cancelled => { output.error("已取消"); return; }
        }
    } else {
        (id, tag.clone(), category.clone())
    };

    let ids = match (id, &tag, &category) {
        (Some(id), None, None) => delete_by_id(id, force, storage, output),
        (None, Some(tags), None) => delete_by_tags(tags, force, storage, output),
        (None, None, Some(cat)) => delete_by_category(cat, force, storage, output),
        _ => {
            output.error("请指定删除目标：笔记ID、标签或分类，且只能指定一种");
            return;
        }
    };

    if ids.is_empty() {
        return;
    }

    for note_id in &ids {
        if let Err(e) = storage.delete_note(*note_id) {
            output.error(format!("删除笔记 {} 失败 - {}", note_id, e));
        }
    }

    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    if ids.len() == 1 {
        output.success(format!("笔记 {} 已删除", ids[0]));
    } else {
        output.success(format!("已删除 {} 条笔记", ids.len()));
    }
}

fn confirm(msg: &str, force: bool) -> bool {
    if force {
        return true;
    }
    Confirm::new()
        .with_prompt(msg)
        .interact()
        .unwrap_or(false)
}

fn delete_by_id(id: u32, force: bool, storage: &mut DataBaseStorage, output: &Output) -> Vec<u32> {
    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return vec![];
    }

    if storage.is_archived(id) {
        output.error(format!("笔记 {} 已归档，无法删除。请先使用 unarchive 取消归档", id));
        return vec![];
    }

    let note = storage.get_note(id);
    let title = note.as_ref().map(|n| n.index.title.as_str()).unwrap_or("未知");
    if !confirm(&format!("确定要删除笔记 [{}] {} 吗？", id, title), force) {
        return vec![];
    }

    vec![id]
}

fn delete_by_tags(tags: &[String], force: bool, storage: &DataBaseStorage, output: &Output) -> Vec<u32> {
    let notes: Vec<_> = storage.list_notes().into_iter()
        .filter(|n| {
            if storage.is_archived(n.id) { return false; }
            let note_tag_names: Vec<&str> = n.tags.iter().map(|t| t.name.as_str()).collect();
            tags.iter().all(|t| note_tag_names.contains(&t.as_str()))
        })
        .collect();

    if notes.is_empty() {
        output.empty(format!("没有包含标签 [{}] 的笔记", tags.join(", ")));
        return vec![];
    }

    if !confirm(&format!("确定要删除 {} 条包含标签 [{}] 的笔记吗？", notes.len(), tags.join(", ")), force) {
        return vec![];
    }

    notes.into_iter().map(|n| n.id).collect()
}

fn delete_by_category(cat: &str, force: bool, storage: &DataBaseStorage, output: &Output) -> Vec<u32> {
    let notes: Vec<_> = storage.list_notes().into_iter()
        .filter(|n| {
            !storage.is_archived(n.id) && n.category.name == cat
        })
        .collect();

    if notes.is_empty() {
        output.empty(format!("分类 '{}' 下没有笔记", cat));
        return vec![];
    }

    if !confirm(&format!("确定要删除分类 '{}' 下的 {} 条笔记吗？", cat, notes.len()), force) {
        return vec![];
    }

    notes.into_iter().map(|n| n.id).collect()
}
