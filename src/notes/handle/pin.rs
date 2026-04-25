use super::super::storage::DataBaseStorage;
use super::super::output::Output;
use super::super::input;

pub fn pin(id: Option<u32>, storage: &mut DataBaseStorage, output: &Output) {
    let id = match id {
        Some(id) => id,
        None => match input::prompt_note_id(storage) {
            Some(id) => id,
            None => { output.error("已取消"); return; }
        }
    };

    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return;
    }
    if storage.is_pinned(id) {
        output.info(format!("笔记 {} 已经置顶", id));
        return;
    }
    storage.pin_note(id);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }
    output.success(format!("笔记 {} 已置顶", id));
}

pub fn unpin(id: Option<u32>, storage: &mut DataBaseStorage, output: &Output) {
    let id = match id {
        Some(id) => id,
        None => match input::prompt_note_id(storage) {
            Some(id) => id,
            None => { output.error("已取消"); return; }
        }
    };

    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return;
    }
    if !storage.is_pinned(id) {
        output.info(format!("笔记 {} 未置顶", id));
        return;
    }
    storage.unpin_note(id);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }
    output.success(format!("笔记 {} 已取消置顶", id));
}
