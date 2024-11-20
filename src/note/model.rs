use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub author_id: String,
    pub last_modified: Option<String>,
}

impl Note {
    pub fn new(title: String, body: String, tags: Vec<String>, author_id: String) -> Result<Self, String> {
        if let Err(err) = Note::validate_title(&title.clone()) {
            return Err(err.to_string());
        }
        if let Err(err) = Note::validate_body(&body.clone()) {
            return Err(err.to_string());
        }
        
        Ok(Self {
            id: None,
            title,
            body: Some(body),
            tags,
            author_id,
            last_modified: None,
        })
    }
    pub fn default() -> Self {
        Note {
            id: None,
            title: "".to_string(),
            body: None,
            tags: vec![],
            author_id: "".to_string(),
            last_modified: None,
        }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn set_last_modified(&mut self, last_modified: String) {
        self.last_modified = Some(last_modified);
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