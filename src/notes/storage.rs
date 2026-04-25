use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use super::model::{NoteStatus, CategoryStatus, TagStatus, NoteIndexModel, NoteModel, CategoryModel, TagModel};
use super::config::StorageConfig;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DataBaseStorage {
    #[serde(skip)]
    notes_dir: PathBuf,
    #[serde(skip)]
    index_file: PathBuf,
    #[serde(default)]
    note_status: NoteStatus,
    #[serde(default)]
    category_status: CategoryStatus,
    #[serde(default)]
    tag_status: TagStatus,
}

impl DataBaseStorage {
    ///加载并且同步notes内容和index
    pub fn init(config: &StorageConfig) -> Result<Self, Box<dyn std::error::Error>> {
        
        fs::create_dir_all(&config.notes_dir)?;
        if let Some(parent) = config.index_file.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut storage = Self::load(config)?;
        storage.sync_notes();
        Ok(storage)
    }

    //从config添加index.jon，加载index索引到内存
    pub fn load(config: &StorageConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut storage = if config.index_file.exists() {
            let content = fs::read_to_string(&config.index_file)?;
            let mut s: Self = serde_json::from_str(&content)?;
            s.notes_dir = config.notes_dir.clone();
            s.index_file = config.index_file.clone();
            s
        } else {
            let s = Self {
                notes_dir: config.notes_dir.clone(),
                index_file: config.index_file.clone(),
                ..Default::default()
            };
            s.save_index()?;
            s
        };
        Ok(storage)
    }


    /// 将磁盘上的笔记文件元数据与 index.json 同步
    fn sync_notes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let disk_names = self.scan_disk_note_names()?;
        let index_ids = self.collect_index_ids();

        let mut changed = false;

        // 磁盘上存在但索引中没有的笔记 → 从文件读取元数据并添加
        for name in &disk_names {
            if let Some(note_index) = self.read_note_metadata(name) {
                if !index_ids.contains(&note_index.id) {
                    self.note_status.notes.push(note_index);
                    changed = true;
                }
            }
        }

        // 索引中存在但磁盘上没有的笔记 → 从索引中移除
        let disk_ids: HashSet<u32> = disk_names.iter().filter_map(|name| {
            self.read_note_metadata(name).map(|m| m.id)
        }).collect();

        let before = self.note_status.notes.len();

        self.note_status.notes.retain(|n| disk_ids.contains(&n.id));
        self.note_status.pinned_notes_id.retain(|id| disk_ids.contains(id));

        let after = self.note_status.notes.len();

        if before != after {
            changed = true;
        }

        // 同步 category_status 和 tag_status
        let mut cat_map: HashMap<u32, CategoryModel> = HashMap::new();
        let mut tag_map: HashMap<u32, TagModel> = HashMap::new();

        for note in &self.note_status.notes {
            cat_map.entry(note.category.id).or_insert_with(|| note.category.clone());
            for tag in &note.tags {
                tag_map.entry(tag.id).or_insert_with(|| tag.clone());
            }
        }

        let old_cat: HashSet<u32> = self.category_status.categories.iter().map(|c| c.id).collect();
        let old_tag: HashSet<u32> = self.tag_status.tags.iter().map(|t| t.id).collect();

        self.category_status.categories = cat_map.into_values().collect();
        self.category_status.categories.sort_by_key(|c| c.id);

        self.tag_status.tags = tag_map.into_values().collect();
        self.tag_status.tags.sort_by_key(|t| t.id);

        let new_cat: HashSet<u32> = self.category_status.categories.iter().map(|c| c.id).collect();
        let new_tag: HashSet<u32> = self.tag_status.tags.iter().map(|t| t.id).collect();

        if old_cat != new_cat || old_tag != new_tag {
            changed = true;
        }

        if changed {
            self.save_index()?;
        }

        Ok(())
    }

    /// 扫描 notes_dir 下所有 .md 文件，返回文件名（不含扩展名）集合
    fn scan_disk_note_names(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut names = Vec::new();
        for entry in fs::read_dir(&self.notes_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    names.push(stem.to_string());
                }
            }
        }
        Ok(names)
    }

    /// 收集索引中所有笔记的 ID
    fn collect_index_ids(&self) -> HashSet<u32> {
        self.note_status
            .notes
            .iter()
            .map(|n| n.id)
            .collect()
    }

    /// 从 .md 文件的 JSON frontmatter 中读取笔记元数据
    fn read_note_metadata(&self, name: &str) -> Option<NoteIndexModel> {
        let path = self.notes_dir.join(format!("{}.md", name));
        let content = fs::read_to_string(&path).ok()?;
        let frontmatter = Self::extract_frontmatter(&content)?;
        serde_json::from_str(&frontmatter).ok()
    }

    /// 从 markdown 内容中提取 frontmatter（--- 之间的部分）
    fn extract_frontmatter(content: &str) -> Option<String> {
        let trimmed = content.trim_start();
        if !trimmed.starts_with("---") {
            return None;
        }

        let after_first = &trimmed[3..];
        let newline_pos = after_first
            .find(|c: char| c == '\n')
            .unwrap_or(after_first.len());
        let rest = &after_first[newline_pos..];

        let end_marker = "\n---";
        let end_pos = rest.find(end_marker)?;
        if end_pos == 0 {
            return Some(String::new());
        }
        Some(rest[1..end_pos].to_string())
    }
    /// 保存到index.json
    pub fn save_index(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&self.index_file, json)?;
        Ok(())
    }

    /// 获取下一个可用的笔记 ID（当前最大 ID + 1）
    pub fn next_note_id(&self) -> u32 {
        self.note_status.notes.iter().map(|n| n.id).max().unwrap_or(0) + 1
    }

    /// 根据名称查找分类，若不存在则自动创建新分类并返回
    pub fn resolve_category(&mut self, name: &str) -> CategoryModel {
        if let Some(existing) = self.category_status.categories.iter().find(|c| c.name == name) {
            existing.clone()
        } else {
            let id = self.category_status.categories.iter().map(|c| c.id).max().unwrap_or(0) + 1;
            let model = CategoryModel { id, name: name.to_string(), parentid: 0 };
            self.category_status.categories.push(model.clone());
            model
        }
    }

    /// 根据名称查找标签，若不存在则自动创建新标签并返回
    pub fn resolve_tag(&mut self, name: &str) -> TagModel {
        if let Some(existing) = self.tag_status.tags.iter().find(|t| t.name == name) {
            existing.clone()
        } else {
            let id = self.tag_status.tags.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let model: TagModel = TagModel { id, name: name.to_string() };
            self.tag_status.tags.push(model.clone());
            model
        }
    }

    /// 将笔记写入磁盘文件（frontmatter + 正文），文件名为标题的净化形式
    pub fn write_note_file(&self, note: &NoteModel) -> std::io::Result<()> {
        let frontmatter = serde_json::to_string(&note.index).expect("序列化笔记索引失败");
        let file_content = format!("---\n{}\n---\n{}", frontmatter, note.content);
        let filename = format!("{}.md", sanitize_filename(&note.index.title));
        let path = self.notes_dir.join(filename);
        fs::write(path, file_content)
    }

    /// 将笔记索引信息添加到内存中的 note_status
    pub fn add_note(&mut self, note: NoteModel) {
        self.note_status.notes.push(note.index);
    }

    /// 检查指定标题的笔记是否已存在
    pub fn title_exists(&self, title: &str) -> bool {
        self.note_status.notes.iter().any(|n| n.title == title)
    }

    /// 返回所有笔记的索引信息引用列表
    pub fn list_notes(&self) -> Vec<&NoteIndexModel> {
        self.note_status.notes.iter().collect()
    }

    /// 根据 ID 获取完整笔记（包含正文内容），从磁盘读取 .md 文件
    pub fn get_note(&self, id: u32) -> Option<NoteModel> {
        let index = self.note_status.notes.iter().find(|n| n.id == id)?;
        let filename = format!("{}.md", sanitize_filename(&index.title));
        let path = self.notes_dir.join(filename);
        let raw = fs::read_to_string(&path).ok()?;
        let body = extract_body(&raw);
        Some(NoteModel {
            index: index.clone(),
            content: body,
        })
    }

    /// 更新笔记内容，先写入新文件成功后再删除旧文件，最后更新索引
    pub fn update_note(&mut self, note: NoteModel) -> std::io::Result<()> {
        let old = self.note_status.notes.iter().find(|n| n.id == note.index.id)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "笔记不存在"))?;

        let old_title = old.title.clone();

        self.write_note_file(&note)?;

        // 新文件写入成功后，再删除旧文件
        if old_title != note.index.title {
            let old_file = self.notes_dir.join(format!("{}.md", sanitize_filename(&old_title)));
            let _ = fs::remove_file(old_file);
        }

        if let Some(existing) = self.note_status.notes.iter_mut().find(|n| n.id == note.index.id) {
            *existing = note.index;
        }
        Ok(())
    }

    /// 根据 ID 删除笔记，同时移除磁盘文件、索引记录、置顶和归档状态
    pub fn delete_note(&mut self, id: u32) -> std::io::Result<()> {
        let index = self.note_status.notes.iter().find(|n| n.id == id)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "笔记不存在"))?;
        let filename = format!("{}.md", sanitize_filename(&index.title));
        fs::remove_file(self.notes_dir.join(filename))?;
        self.note_status.notes.retain(|n| n.id != id);
        self.note_status.pinned_notes_id.retain(|&pid| pid != id);
        self.note_status.archived_notes.retain(|&aid| aid != id);
        self.note_status.done_notes.retain(|&did| did != id);
        Ok(())
    }

    /// 将指定笔记设为置顶（重复置顶不重复添加）
    pub fn pin_note(&mut self, id: u32) {
        if !self.note_status.pinned_notes_id.contains(&id) {
            self.note_status.pinned_notes_id.push(id);
        }
    }

    /// 取消指定笔记的置顶状态
    pub fn unpin_note(&mut self, id: u32) {
        self.note_status.pinned_notes_id.retain(|&pid| pid != id);
    }

    /// 检查指定笔记是否处于置顶状态
    pub fn is_pinned(&self, id: u32) -> bool {
        self.note_status.pinned_notes_id.contains(&id)
    }

    /// 将指定笔记归档（重复归档不重复添加）
    pub fn archive_note(&mut self, id: u32) {
        if !self.note_status.archived_notes.contains(&id) {
            self.note_status.archived_notes.push(id);
        }
    }

    /// 取消指定笔记的归档状态
    pub fn unarchive_note(&mut self, id: u32) {
        self.note_status.archived_notes.retain(|&aid| aid != id);
    }

    /// 检查指定笔记是否已归档
    pub fn is_archived(&self, id: u32) -> bool {
        self.note_status.archived_notes.contains(&id)
    }

    /// 将指定笔记标记为已完成（重复标记不重复添加）
    pub fn done_note(&mut self, id: u32) {
        if !self.note_status.done_notes.contains(&id) {
            self.note_status.done_notes.push(id);
        }
    }

    /// 检查指定笔记是否已完成
    pub fn is_done(&self, id: u32) -> bool {
        self.note_status.done_notes.contains(&id)
    }

    /// 返回所有分类的引用
    pub fn list_categories(&self) -> &Vec<CategoryModel> {
        &self.category_status.categories
    }

    /// 重命名分类，同步更新所有关联笔记的分类名称、磁盘文件和分类列表
    pub fn rename_category(&mut self, old_name: &str, new_name: &str) {
        for note in &mut self.note_status.notes {
            if note.category.name == old_name {
                note.category.name = new_name.to_string();
                note.modified = chrono::Local::now();
            }
        }
        for cat in &mut self.category_status.categories {
            if cat.name == old_name {
                cat.name = new_name.to_string();
            }
        }

        // 同步磁盘文件：重新写入受影响的笔记
        for note_index in &self.note_status.notes {
            if note_index.category.name == new_name {
                if let Some(note) = self.get_note(note_index.id) {
                    let _ = self.write_note_file(&note);
                }
            }
        }
        let _ = self.save_index();
    }

    /// 删除分类，保留笔记文件并重置为默认分类
    pub fn delete_category_keep_notes(&mut self, name: &str) {
        self.category_status.categories.retain(|c| c.name != name);
        for note in &mut self.note_status.notes {
            if note.category.name == name {
                note.category = CategoryModel { id: 0, name: "default".to_string(), parentid: 0 };
                note.modified = chrono::Local::now();
            }
        }

        // 同步磁盘文件：更新受影响笔记的 frontmatter
        for note_index in &self.note_status.notes.clone() {
            if note_index.category.name == "default" {
                if let Some(note) = self.get_note(note_index.id) {
                    let _ = self.write_note_file(&note);
                }
            }
        }
        let _ = self.save_index();
    }

    /// 删除分类及其下所有笔记文件
    pub fn delete_category_with_notes(&mut self, name: &str) {
        let ids_to_delete: Vec<u32> = self.note_status.notes.iter()
            .filter(|n| n.category.name == name)
            .map(|n| n.id)
            .collect();

        for id in &ids_to_delete {
            let _ = self.delete_note(*id);
        }

        self.category_status.categories.retain(|c| c.name != name);
        let _ = self.save_index();
    }

    /// 返回所有标签的引用
    pub fn list_tags(&self) -> &Vec<TagModel> {
        &self.tag_status.tags
    }

    /// 重命名标签，同步更新所有关联笔记的标签名称、磁盘文件和标签列表
    pub fn rename_tag(&mut self, old_name: &str, new_name: &str) {
        for note in &mut self.note_status.notes {
            for tag in &mut note.tags {
                if tag.name == old_name {
                    tag.name = new_name.to_string();
                    note.modified = chrono::Local::now();
                }
            }
        }
        for tag in &mut self.tag_status.tags {
            if tag.name == old_name {
                tag.name = new_name.to_string();
            }
        }

        // 同步磁盘文件：重新写入受影响的笔记
        for note_index in &self.note_status.notes {
            if note_index.tags.iter().any(|t| t.name == new_name) {
                if let Some(note) = self.get_note(note_index.id) {
                    let _ = self.write_note_file(&note);
                }
            }
        }
        let _ = self.save_index();
    }

    /// 删除标签，同时从所有笔记中移除该标签的关联
    pub fn delete_tag(&mut self, name: &str) {
        self.tag_status.tags.retain(|t| t.name != name);
        for note in &mut self.note_status.notes {
            note.tags.retain(|t| t.name != name);
        }
    }

    /// 根据 ID 获取笔记的索引信息（不可变引用）
    pub fn get_note_index(&self, id: u32) -> Option<&NoteIndexModel> {
        self.note_status.notes.iter().find(|n| n.id == id)
    }

    /// 根据 ID 获取笔记的索引信息（可变引用）
    pub fn get_note_index_mut(&mut self, id: u32) -> Option<&mut NoteIndexModel> {
        self.note_status.notes.iter_mut().find(|n| n.id == id)
    }

    /// 返回笔记状态的不可变引用
    pub fn note_status_ref(&self) -> &NoteStatus {
        &self.note_status
    }

    /// 检查指定 ID 的笔记是否存在
    pub fn id_exists(&self, id: u32) -> bool {
        self.note_status.notes.iter().any(|n| n.id == id)
    }
}

/// 将标题中的非法文件名字符替换为下划线，并去除首尾的点和空格；结果为空则返回 "untitled"
fn sanitize_filename(title: &str) -> String {
    let sanitized: String = title
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect();
    let trimmed = sanitized.trim_matches(|c: char| c == '.' || c == ' ');
    if trimmed.is_empty() {
        "untitled".to_string()
    } else {
        trimmed.to_string()
    }
}

/// 从 markdown 原始内容中提取正文部分（去除 frontmatter 之后的内容）
fn extract_body(raw: &str) -> String {
    let trimmed = raw.trim_start();
    if !trimmed.starts_with("---") {
        return raw.to_string();
    }
    let rest = &trimmed[3..];
    let after_fm = match rest.find("\n---") {
        Some(pos) => &rest[pos + 4..],
        None => return String::new(),
    };
    after_fm.trim().to_string()
}
