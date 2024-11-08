use cfg_if::cfg_if;

pub mod tag;

cfg_if! {
    if #[cfg(feature="ssr")] {
        // pub mod app_handler;
        pub mod auth;
        pub mod config;
    }
}

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










pub enum EditorStatus {
    View,
    Contribute,
    Maintain
}

impl EditorStatus {
    pub fn new() -> Self {
        EditorStatus::View
    }
    
    pub fn is_read_only(&self) -> bool {
        match self {
            EditorStatus::View => true,
            _ => false
        }
    }

    pub fn contribute(&self) -> Result<Self, &str> {
        match self {
            EditorStatus::View => Ok(EditorStatus::Contribute),
            _ => Err("Cannot contribute")
        }
    }
    pub fn maintain(&self) -> Result<Self, &str> {
        match self {
            EditorStatus::View => Ok(EditorStatus::Maintain),
            _ => Err("Cannot maintain")
        }
    }
    pub fn commit(&self) -> Result<(), &str> {
        Ok(())
    }
}