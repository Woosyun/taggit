use serde::{Serialize, Deserialize};
use crate::note::Note;

#[derive(Serialize, Deserialize)]
pub struct NoteItem {
    pub title: String,
    pub id: Option<String>,
    pub author_id: String,
    pub last_modified: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl From<Note> for NoteItem {
    fn from(note: Note) -> NoteItem {
        NoteItem {
            title: note.title,
            id: note.id,
            author_id: note.author_id,
            last_modified: note.last_modified,
            tags: note.tags
        }
    }
}