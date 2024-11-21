use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub author_id: String,
    pub body: String,
    pub last_modified: Option<String>,
    pub target_id: String,
    pub target_position: Option<String>,
}

impl Comment {
    pub fn new(author_id: String, body: String, target_id: String, target_position: Option<String>) -> Self {
        Self {
            id: None,
            author_id,
            body,
            last_modified: None,
            target_id,
            target_position,
        }
    }
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id)
    }
    pub fn set_last_modified(&mut self, last_modified: String) {
        self.last_modified = Some(last_modified)
    }
}