use super::super::storage::DataBaseStorage;
use super::super::output::Output;

pub fn handle(id: u32, storage: &mut DataBaseStorage, output: &Output) {
    if !storage.id_exists(id) {
        output.error(format!("笔记ID {} 不存在", id));
        return;
    }

    storage.done_note(id);
    if let Err(e) = storage.save_index() {
        output.error(format!("保存索引失败 - {}", e));
        return;
    }

    output.success(format!("笔记 {} 已标记为完成", id));
}
