use leptos::{
    prelude::*, 
    html
};
use leptos_router::hooks::use_query_map;

#[component]
pub fn CreateNotePage() -> impl IntoView {
    use leptos::logging::log;
    use web_sys::window;

    let query = use_query_map();
    let tags = move || query
        .get()
        .get_all("tags")
        .unwrap_or_default();
    let title_ref: NodeRef<html::Input> = NodeRef::new();
    let body_ref: NodeRef<html::Textarea> = NodeRef::new();

    let create_note = Action::new(move |&()| {
        let title = title_ref.get().expect("title_ref to exist").value();
        let body = body_ref.get().expect("body_ref to exist").value();
        let tags = tags();

        async move {
            insert_note(title, body, Some(tags)).await
                .map_err(|e| {
                    let f = format!("error while creating note: {e:?}");
                    window().unwrap().alert_with_message(&f).unwrap();
                })
            }
        }
    );
    
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

        <form 
            class="note-container" 
            on:submit=move |ev| {
                ev.prevent_default();
                create_note.dispatch(());
            }
        >
            <input type="text" placeholder="title" node_ref=title_ref/>
            <textarea class="note-body" node_ref=body_ref></textarea>
            <button type="submit">submit</button>
        </form>
    }
}


#[server(InsertNote)]
pub async fn insert_note(
    title: String, 
    body: String, 
    tags: Option<Vec<String>>,
) -> Result<String, ServerFnError> {
    use leptos::prelude::use_context;
    use leptos_axum::{extract, redirect};
    use axum_login::AuthSession;
    use crate::{
        app::AppState, 
        auth::Backend,
        note::Note,
    };

    let auth_session: AuthSession<Backend> = extract().await?;
    let note_service = use_context::<AppState>()
        .unwrap()
        .db
        .note_service;

    let tags = tags.unwrap_or_default();
    let user_id = auth_session.user
        .map(|user| user.user_id)
        .ok_or(ServerFnError::new("Unauthorized"))?;

    let new_note = Note::new(
        title, 
        body, 
        tags, 
        user_id,
    )
    .map_err(|e| dbg!(e))
    .map_err(ServerFnError::new)?;

    note_service
        .insert_one(new_note)
        .await
        .map_err(|e| dbg!(e))
        .map_err(ServerFnError::new)
        .map(|insert_one_result| {
            redirect("/");
            insert_one_result.inserted_id.to_string()
        })
}