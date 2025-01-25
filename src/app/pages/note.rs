use leptos::prelude::*;
use crate::note::{Element, Note};

#[component] 
pub fn Page() -> impl IntoView {
    let mut note = Note::default();
    note.body = vec![
        Element::H1("hello world".to_string()),
        Element::P("paragraph1".to_string()),
        Element::P("paragraph2".to_string()),
    ];

    view! {
        {note}
    }
}
