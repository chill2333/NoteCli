use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn handle(id: u32, force: bool, storage: &mut DataBaseStorage, output: &Output) {
    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return;
    }

    if storage.is_archived(id) {
        output.error(format!("笔记 {} 已归档，无法删除。请先使用 unarchive 取消归档", id));
        return;
    }

    if !force {
        output.info(format!("确定要删除笔记 {} 吗？使用 -f 跳过确认", id));
        return;
    }

    if let Err(e) = storage.delete_note(id) {
        output.error(format!("删除笔记失败 - {}", e));
        return;
    }

    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("笔记 {} 已删除", id));
}
