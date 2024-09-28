use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct DBPath(pub std::path::PathBuf);

#[derive(Debug, Serialize)]

pub struct Note {
    pub id: u64,
    pub title: String,
    pub last_modified: u64,
    pub content: String
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug, Serialize)]
pub struct Tag {
    pub id: u64,
    pub name: String
}

#[allow(dead_code)]
pub struct TagNote {
    note_id: u64,
    tags: Vec<String>
}