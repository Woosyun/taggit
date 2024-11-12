use leptos::{
    ev::SubmitEvent, 
    prelude::*, 
    html
};
use leptos_router::hooks::use_query_map;
use crate::note::Note;

#[component]
pub fn CreateNotePage() -> impl IntoView {
    use web_sys::window;
    let window = window().unwrap();

    let query = use_query_map();
    let tags = move || query
        .get()
        .get_all("tags")
        .unwrap_or_else(|| vec![]);

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
            let id = insert_note(title, body, tags, author_id).await;
            match id {
                Ok(id) => {
                    log!("(CreateNotePage) inserted note with id: {:?}", id);
                }
                Err(err) => {
                    log!("(CreateNotePage) error while inserting note: {:?}", err);
                }
            }
        }
    });
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        //TODO: pass validation to server??
        
        let title = title_ref.get().expect("title_ref to exist").value();
        if let Err(err) = Note::validate_title(&title) {
            window.alert_with_message(err).unwrap();
            return;
        }

        let body = body_ref.get().expect("body_ref to exist").value();
        if let Err(err) = Note::validate_body(&body) {
            window.alert_with_message(err).unwrap();
            return;
        }

        let author_id = "admin".to_string();

        insert_note.dispatch((title, body, tags(), author_id));

        ()
    };
    
    view! {
        <div class="tagbar">
            <For each=tags key=|tag| tag.clone() children=move |tag: String| {
                let tag0 = tag.clone();
                view! {
                    <span class="badge" on:click=move |ev| {
                        ev.prevent_default();
                        log!("deleting tag: {:?}", tag0.clone());
                    }>{tag}</span>
                }
            } />
        </div>

        <form class="note-container" on:submit=on_submit>
            <input type="text" placeholder="title" node_ref=title_ref/>
            <textarea class="note-body" node_ref=body_ref></textarea>
            <button type="submit">submit</button>
        </form>
    }
}


#[server(InsertNote, "/api")]
pub async fn insert_note(
    title: String, 
    body: String, 
    tags: Vec<String>, 
    author_id: String
) -> Result<String, ServerFnError<String>> {
    use leptos::prelude::{log, use_context};
    use crate::app::AppState;

    // let note_service = use_context::<DB>()
    //     .map(|db| db.note_service)
    //     .ok_or(ServerFnError::ServerError("cannot connect to database".to_string()))?;
    let note_service = use_context::<AppState>()
        .unwrap()
        .db
        .note_service;

    let new_note = Note::new(
        None,
        title, 
        body, 
        tags, 
        author_id, 
        vec![]
    ).map_err(|err| ServerFnError::ServerError(err))?;

    log!("(api/insert_note) got new note: {:?}", new_note);

    note_service
        .insert_one(new_note)
        .await
        .map_err(|err| ServerFnError::ServerError(format!("error while inserting note: {:?}", err)))
        .map(|insert_one_result| insert_one_result.inserted_id.to_string())
}