use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct DBPath(pub std::path::PathBuf);

#[derive(Debug, Serialize)]

pub struct Note {
    pub id: i64,
    pub title: String,
    pub last_modified: String,
    pub content: String
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Debug, Serialize)]
pub struct Tag {
    pub id: i64,
    pub name: String
}

#[allow(dead_code)]
pub struct TagNote {
    note_id: i64,
    tags: Vec<String>
}

//todo: implement datetime type to store last_modified in unix epoch and offer method to get current local time stamp