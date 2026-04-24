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

    /// 读取完整的笔记（元数据 + 正文内容）
    pub fn read_note(&self, id: u32) -> Option<NoteModel> {
        let path = self.notes_dir.join(format!("{id}.md"));
        let content = fs::read_to_string(&path).ok()?;

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
        let frontmatter = &rest[1..end_pos];
        let body = &rest[end_pos + end_marker.len()..];

        let index = serde_json::from_str(frontmatter).ok()?;

        Some(NoteModel {
            index,
            content: body.trim_start().to_string(),
        })
    }
    /// 保存到index.json
    pub fn save_index(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&self.index_file, json)?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use super::super::model::{Priority, CategoryModel};
    use chrono::Local;

    fn make_config(dir: &TempDir) -> StorageConfig {
        StorageConfig {
            notes_dir: dir.path().join("notes"),
            index_file: dir.path().join("index.json"),
        }
    }

    fn make_note_content(id: u32, title: &str, priority: &str) -> String {
        format!(
            "---\n\
{{\"id\":{id},\"title\":\"{title}\",\
\"category\":{{\"id\":1,\"name\":\"test\",\"parentid\":0}},\
\"tags\":[{{\"id\":1,\"name\":\"tag1\"}}],\
\"priority\":\"{priority}\",\
\"created\":\"2026-04-24T10:00:00+08:00\",\
\"modified\":\"2026-04-24T10:00:00+08:00\"}}\n\
---\n\
这是 {title} 的正文内容。\n"
        )
    }

    // ---- extract_frontmatter 测试 ----

    #[test]
    fn test_extract_frontmatter_valid() {
        let content = "---\n{\"id\":1,\"title\":\"test\"}\n---\nbody";
        let result = DataBaseStorage::extract_frontmatter(content);
        assert!(result.is_some());
        let fm = result.unwrap();
        assert!(fm.contains("\"id\":1"));
        assert!(fm.contains("\"title\":\"test\""));
    }

    #[test]
    fn test_extract_frontmatter_no_frontmatter() {
        let content = "没有 frontmatter 的内容";
        let result = DataBaseStorage::extract_frontmatter(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_frontmatter_empty_frontmatter() {
        let content = "---\n---\nbody here";
        let result = DataBaseStorage::extract_frontmatter(content);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }

    // ---- scan_disk_notes 测试 ----

    #[test]
    fn test_scan_disk_notes_finds_md_files() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        fs::write(config.notes_dir.join("note-a.md"), "content").unwrap();
        fs::write(config.notes_dir.join("note-b.md"), "content").unwrap();
        fs::write(config.notes_dir.join("readme.txt"), "content").unwrap();

        let storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        let ids = storage.scan_disk_note_names().unwrap();
        assert_eq!(ids.len(), 2);
        assert!(ids.iter().any(|s| s == "note-a"));
        assert!(ids.iter().any(|s| s == "note-b"));
        assert!(!ids.iter().any(|s| s == "readme"));
    }

    #[test]
    fn test_scan_disk_notes_empty_dir() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        let storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        let ids = storage.scan_disk_note_names().unwrap();
        assert!(ids.is_empty());
    }

    // ---- read_note_metadata 测试 ----

    #[test]
    fn test_read_note_metadata_valid() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        let note_content = make_note_content(1, "测试笔记", "Normal");
        fs::write(config.notes_dir.join("1.md"), &note_content).unwrap();

        let storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        let meta = storage.read_note_metadata("1").unwrap();
        assert_eq!(meta.id, 1);
        assert_eq!(meta.title, "测试笔记");
        assert_eq!(meta.priority, Priority::Normal);
    }

    #[test]
    fn test_read_note_metadata_missing_file() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        let storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        assert!(storage.read_note_metadata("nonexistent").is_none());
    }

    #[test]
    fn test_read_note_metadata_no_frontmatter() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        fs::write(config.notes_dir.join("bad.md"), "没有 frontmatter").unwrap();

        let storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        assert!(storage.read_note_metadata("bad").is_none());
    }

    // ---- sync_notes 测试 ----

    #[test]
    fn test_sync_adds_new_notes_to_index() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        fs::write(
            config.notes_dir.join("1.md"),
            make_note_content(1, "新笔记", "High"),
        ).unwrap();

        let index_path = config.index_file.clone();

        let mut storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        storage.sync_notes().unwrap();

        assert_eq!(storage.note_status.notes.len(), 1);
        assert_eq!(storage.note_status.notes[0].id, 1);
        assert_eq!(storage.note_status.notes[0].title, "新笔记");

        assert!(index_path.exists());
    }

    #[test]
    fn test_sync_removes_orphaned_index_entries() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        let ghost_index = NoteIndexModel {
            id: 999,
            title: "幽灵笔记".to_string(),
            category: CategoryModel { id: 1, name: "test".to_string(), parentid: 0 },
            tags: vec![],
            priority: Priority::Normal,
            created: Local::now(),
            modified: Local::now(),
        };

        let mut storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            note_status: NoteStatus {
                notes: vec![ghost_index],
                ..Default::default()
            },
            ..Default::default()
        };

        storage.sync_notes().unwrap();

        assert!(storage.note_status.notes.is_empty());
    }

    #[test]
    fn test_sync_skips_file_without_frontmatter() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        fs::write(
            config.notes_dir.join("1.md"),
            make_note_content(1, "有效笔记", "Normal"),
        ).unwrap();
        fs::write(config.notes_dir.join("bad.md"), "纯文本无元数据").unwrap();

        let mut storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        storage.sync_notes().unwrap();

        assert_eq!(storage.note_status.notes.len(), 1);
        assert_eq!(storage.note_status.notes[0].id, 1);
    }

    // ---- init 集成测试 ----

    #[test]
    fn test_init_creates_index_when_missing() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);

        let storage = DataBaseStorage::init(&config).unwrap();

        assert!(config.index_file.exists());
        assert!(storage.note_status.notes.is_empty());
    }

    #[test]
    fn test_init_syncs_disk_notes_into_empty_index() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        fs::write(
            config.notes_dir.join("1.md"),
            make_note_content(1, "笔记1", "Normal"),
        ).unwrap();
        fs::write(
            config.notes_dir.join("2.md"),
            make_note_content(2, "笔记2", "Urgent"),
        ).unwrap();

        let storage = DataBaseStorage::init(&config).unwrap();

        assert_eq!(storage.note_status.notes.len(), 2);
    }

    #[test]
    fn test_init_loads_existing_index_and_syncs() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        fs::write(
            config.notes_dir.join("1.md"),
            make_note_content(1, "已有笔记", "Low"),
        ).unwrap();

        let storage1 = DataBaseStorage::init(&config).unwrap();
        assert_eq!(storage1.note_status.notes.len(), 1);

        fs::write(
            config.notes_dir.join("2.md"),
            make_note_content(2, "额外笔记", "High"),
        ).unwrap();

        let storage2 = DataBaseStorage::init(&config).unwrap();
        assert_eq!(storage2.note_status.notes.len(), 2);
    }

    // ---- read_note 测试 ----

    #[test]
    fn test_read_note_returns_metadata_and_content() {
        let dir = TempDir::new().unwrap();
        let config = make_config(&dir);
        fs::create_dir_all(&config.notes_dir).unwrap();

        let content = make_note_content(1, "完整笔记", "Normal");
        fs::write(config.notes_dir.join("1.md"), &content).unwrap();

        let storage = DataBaseStorage {
            notes_dir: config.notes_dir,
            index_file: config.index_file,
            ..Default::default()
        };

        let note = storage.read_note(1).unwrap();
        assert_eq!(note.index.id, 1);
        assert_eq!(note.index.title, "完整笔记");
        assert!(note.content.contains("完整笔记 的正文内容"));
    }
}
