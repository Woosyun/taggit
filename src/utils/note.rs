use mongodb::bson::oid::ObjectId;
use serde::*;

pub struct Note {
    id: ObjectId,
    title: String,
    body: String,
    tags: Vec<String>,
    author: ObjectId,
}

impl Note {
    pub fn open(id: ObjectId) -> Note {
        // open note by id
        Note {
            id: ObjectId::new(),
            title: "Note Title".to_string(),
            body: "Note Body".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            author: ObjectId::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NoteItem {
    id: ObjectId,
    title: String,
    author: ObjectId
}

impl NoteItem {
    pub fn search_by_tags(tags: Vec<String>) -> Vec<NoteItem> {
        // search notes by tags
        vec![]
    }
}