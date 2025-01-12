use leptos::prelude::*;
use serde::{Serialize, Deserialize};
use crate::note::Element;

#[derive(Serialize, Deserialize, Default)]
pub struct Note {
    pub title: String,
    pub id: Option<String>,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub last_modified: Option<String>,
    #[serde(default)]
    pub body: Vec<Element>, // maybe Vec<NodeRef<Div>>
    #[serde(default)]
    pub tags: Vec<String>,
}
impl Note {
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
    pub fn set_author_id(&mut self, id: String) {
        self.author_id = id;
    }
    pub fn update_last_modified(&mut self, date: String) {
        self.last_modified = Some(date);
    }
}

impl IntoRender for Note {
    type Output = Vec<AnyView>;

    fn into_render(self) -> Self::Output {
        self.body.into_iter().map(IntoRender::into_render).collect_view()
    }
}