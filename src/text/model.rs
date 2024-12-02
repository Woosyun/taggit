use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Text {
    #[serde(rename="_id")]
    pub id: Option<String>,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub body: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_modified: Option<String>,
}

impl Text {
    pub fn new(
        id: Option<String>, 
        author_id: String, 
        parent_id: Option<String>, 
        title: String, 
        body: Option<String>, 
        tags: Vec<String>, 
        last_modified: Option<String>
    ) -> Self {
        Self {
            id,
            author_id,
            parent_id,
            title,
            body,
            tags,
            last_modified,
        }
    }
    
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
    pub fn update_last_modified(&mut self, date: String) {
        self.last_modified = Some(date);
    }
}