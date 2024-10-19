use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    id: String,
    title: String,
    body: String,
    tags: Vec<String>,
    author: String,
    last_modified: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteItem {
    id: String,
    pub title: String,
    pub author: String,
    pub last_modified: u64,
}

impl From<Note> for NoteItem {
    fn from(note: Note) -> Self {
        NoteItem {
            id: note.id,
            title: note.title,
            author: note.author,
            last_modified: note.last_modified,
        }
    }
}