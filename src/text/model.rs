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
    pub fn set_author_id(&mut self, id: String) {
        self.author_id = id
    }
    pub fn update_last_modified(&mut self, date: String) {
        self.last_modified = Some(date);
    }
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Item {
    pub id: Option<String>,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_modified: Option<String>,
}

impl From<Text> for Item {
    fn from(text: Text) -> Self {
        Item {
            id: text.id,
            author_id: text.author_id,
            parent_id: text.parent_id,
            title: text.title,
            tags: text.tags,
            last_modified: text.last_modified,
        }
    }
}