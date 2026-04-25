use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct NoteIndexModel {
    pub id: u32,
    pub title: String,
    pub category: CategoryModel,
    pub tags: Vec<TagModel>,
    pub priority: Priority,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
}

pub struct NoteModel{
    pub index: NoteIndexModel,
    pub content: String,
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Priority {
    Low,
    Normal,
    High,
    Urgent,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CategoryModel{
    pub id:u32,
    pub name:String,
    pub parentid:u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TagModel{
    pub id:u32,
    pub name:String,
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NoteStatus {
    pub notes: Vec<NoteIndexModel>,
    #[serde(default)]
    pub pinned_notes_id: Vec<u32>,
    #[serde(default)]
    pub archived_notes: Vec<u32>,
    #[serde(default)]
    pub done_notes: Vec<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CategoryStatus {
    pub categories: Vec<CategoryModel>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TagStatus {
    pub tags: Vec<TagModel>,
}

