use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: CategoryModel,
    pub tags: Vec<TagModel>,
    pub priority: Priority,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CategoryModel{
    id:u32,
    name:String,
    parentid:u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TagModel{
    id:u32,
    name:String,
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoteStatus {
    pub notes: Vec<NoteModel>,
    pub pinned_notes: Vec<NoteModel>,
    pub archived_notes: Vec<NoteModel>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CategoryStatus {
    pub categories: Vec<CategoryModel>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagStatus {
    pub tags: Vec<TagModel>,
}

