use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename="_id")]
    pub id: Option<String>,
    pub user_id: String,
    pub password: String,
    pub user_name: String,
    pub posts: Vec<String>,
    pub comments: Vec<(String, String)>, //(note id, comment id),
}

impl User {
    pub fn new(user_id: String, password: String, user_name: String) -> Self {
        Self {
            id: None,
            user_id,
            password,
            user_name,
            posts: vec![],
            comments: vec![],
        }
    }
    pub fn set_id(&mut self, new_id: String) {
        self.id = Some(new_id);
    }
}

#[derive(Clone, Debug)]
pub struct PublicUserProfile {
    pub authenticated: bool,
}