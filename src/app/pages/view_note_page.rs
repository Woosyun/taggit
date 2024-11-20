use leptos::prelude::*;
use leptos_router::{
    hooks::use_params,
    params::Params
};
use crate::note::Note;

#[derive(Clone, PartialEq, Params)]
struct ContactParams {
    id: String
}

#[component] 
pub fn NoteViewPage() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let id = move || match params.get() {
        Ok(p) => Some(p.id),
        _ => None
    };

    let note = Resource::new(id, |id| async move {
        fetch_note_by_id(id).await
    });
    let note = move || note.get()
        .map(|note| {
            note.map(|note| {
                view! {
                    <p>"note id" {note.id}</p>
                    <p>"note author id" {note.author_id}</p>
                    <p>"note tags" {note.tags}</p>
                    <p>"last modified" {note.last_modified}</p>
                    <h1>{note.title}</h1>
                    <div>{note.body}</div>
                }
            })
        });

    view! {
        <Suspense
            fallback=move || view! {<p>"loading..."</p>}
        >
            <ErrorBoundary
                fallback=move |_| view! {<p>"error while fetching note"</p>}
            >
                {note}
            </ErrorBoundary>
            // {move || note.get().map(|note| {
            //     view! {
            //         <p>{note.author_id}</p>
            //         <h1>{note.title}</h1>
            //         <div>{note.body}</div>
            //     }
            // })}
        </Suspense>
    }
}

#[server(FetchNoteById)]
pub async fn fetch_note_by_id(id: Option<String>) -> Result<Note, ServerFnError> {
    //check if the note exists
    use leptos::prelude::use_context;
    use crate::app::AppState;
    
    let id = match id {
        Some(id) => id,
        None => return Err(ServerFnError::ServerError("unvalid id".to_string()))
    };

    let note_service = use_context::<AppState>()
        .unwrap()
        .db
        .note_service;

    match note_service.find_one_by_id(id).await {
        Ok(note_option) => {
            match note_option {
                Some(note) => Ok(note),
                None => Err(ServerFnError::ServerError("note not found".to_string()))
            }
        },
        Err(err) => Err(ServerFnError::ServerError(format!("error while fetching note: {:?}", err)))
    }
}