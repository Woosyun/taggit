use serde::{Serialize, Deserialize};
use leptos::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    #[serde(rename="_id")]
    pub id: String,
    pub author_id: String,
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_modified: Option<String>,
}

impl Mail {
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn update_last_modified(&mut self, date: String) {
        self.last_modified = Some(date);
    }
}

impl IntoRender for Mail {
    type Output = AnyView;

    fn into_render(self) -> AnyView {
        view! {
            <span>{self.author_id}</span>
            <p>{self.body}</p>
            <span>{self.last_modified}</span>
        }.into_any()
    }
}
