use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn pin(id: u32, storage: &mut DataBaseStorage, output: &Output) {
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

pub fn unpin(id: u32, storage: &mut DataBaseStorage, output: &Output) {
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
