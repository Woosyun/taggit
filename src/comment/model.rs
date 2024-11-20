use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    #[serde(rename = "_id")]
    id: Option<String>,
    author_id: String,
    body: String,
    last_modified: String,
    target_id: String,
    target_position: Option<String>,
}

impl Comment {
    pub fn new(author_id: String, body: String, last_modified: String, target_id: String, target_position: Option<String>) -> Self {
        Self {
            id: None,
            author_id,
            body,
            last_modified,
            target_id,
            target_position,
        }
    }
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id)
    }
    pub fn set_last_modified(&mut self, last_modified: String) {
        self.last_modified = last_modified
    }
}