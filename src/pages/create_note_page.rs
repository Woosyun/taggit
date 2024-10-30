use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::html;
use crate::{
    api, 
    models,
};

#[allow(unused_variables, unused_imports)]
#[component]
pub fn CreateNotePage() -> impl IntoView {
    use web_sys::window;
    let window = window().unwrap();

    let title_ref: NodeRef<html::Input> = NodeRef::new();
    // let title_ref = create_node_ref::<html::Input>();
    let body_ref: NodeRef<html::Textarea> = NodeRef::new();
    // let body_ref = create_node_ref::<html::Textarea>();

    let insert_note = Action::new(|input: &(String, String, Vec<String>, String)| {
        let title = input.0.to_owned();
        let body = input.1.to_owned();
        let tags = input.2.to_owned();
        let author_id = input.3.to_owned();

        log!("(CreateNotePage) inserting note with title: {}, body: {}, tags: {:?}, author_id: {:?}.", &title, &body, &tags, &author_id);
        
        async move {
            api::insert_note(title, body, tags, author_id).await
        }
    });
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let title = title_ref.get().expect("title_ref to exist").value();
        if let Err(err) = models::Note::validate_title(&title) {
            window.alert_with_message(err).unwrap();
            return;
        }

        let body = body_ref.get().expect("body_ref to exist").value();
        if let Err(err) = models::Note::validate_body(&body) {
            window.alert_with_message(err).unwrap();
            return;
        }

        let tags: Vec<String> = vec![];
        let author_id = "admin".to_string();

        insert_note.dispatch((title, body, tags, author_id));

        ()
    };
    
    view! {
        <form class="note-container" on:submit=on_submit>
            <input type="text" placeholder="title" node_ref=title_ref/>
            <textarea class="note-body" node_ref=body_ref></textarea>
            <button type="submit">submit</button>
        </form>
    }
}
