
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub author_id: String,
    pub last_modified: String,
    pub comments: Option<Vec<Comment>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub body: String,
    pub last_modified: String,
}


impl Note {
    pub fn new(id: Option<String>, title: String, body: String, tags: Vec<String>, author_id: String, comments:Vec<Comment>) -> Result<Self, String> {
        if let Err(err) = Note::validate_title(&title.clone()) {
            return Err(err.to_string());
        }
        if let Err(err) = Note::validate_body(&body.clone()) {
            return Err(err.to_string());
        }
        
        Ok(Note {
            id,
            title,
            body: Some(body),
            tags,
            author_id,
            last_modified: "0000-00-00".to_string(),
            comments: Some(comments),
        })
    }
    pub fn default() -> Self {
        Note {
            id: None,
            title: "".to_string(),
            body: None,
            tags: vec![],
            author_id: "".to_string(),
            last_modified: "".to_string(),
            comments: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn set_last_modified(&mut self, last_modified: String) {
        self.last_modified = last_modified;
    }

    pub fn validate_title(title: &str) -> Result<(), &str> {
        if title.is_empty() {
            return Err("title is empty!!")
        }

        Ok(())
    }
    pub fn validate_body(body: &str) -> Result<(), &str> {
        if body.is_empty() {
            return Err("body is empty!!")
        }

        Ok(())
    }
}