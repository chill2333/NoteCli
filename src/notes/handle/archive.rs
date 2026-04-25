use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn archive(id: u32, storage: &mut DataBaseStorage, output: &Output) {
    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return;
    }
    if storage.is_archived(id) {
        output.info(format!("笔记 {} 已归档", id));
        return;
    }
    storage.archive_note(id);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }
    output.success(format!("笔记 {} 已归档", id));
}

pub fn unarchive(id: u32, storage: &mut DataBaseStorage, output: &Output) {
    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return;
    }
    if !storage.is_archived(id) {
        output.info(format!("笔记 {} 未归档", id));
        return;
    }
    storage.unarchive_note(id);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }
    output.success(format!("笔记 {} 已取消归档", id));
}
