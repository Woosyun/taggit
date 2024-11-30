use leptos::prelude::*;
use leptos::html;
use leptos_router::hooks::use_query_map;
use crate::app::utils::get_tags_from_query;

#[component] 
pub fn CreateCommitPage() -> impl IntoView {
    use web_sys::window;
    
    let query = use_query_map();
    let tags = move || get_tags_from_query(query);
    let title_ref: NodeRef<html::Input> = NodeRef::new();
    let body_ref: NodeRef<html::Textarea> = NodeRef::new();

    let create_commit = Action::new(move |&()| {
        let title = title_ref.get().expect("missing title_ref").value();
        let body = body_ref.get().expect("missing body_ref").value();

        let f = format!("title: {:?} body: {:?}", title, body);
        window().unwrap().alert_with_message(&f).unwrap();

        async move {
            insert_commit(title, body, None, Some(tags())).await
                .map_err(|e| {
                    dbg!(e)
                })
        }
    });
    
    view! {
        <form class="container" on:submit=move |ev| {
            ev.prevent_default();
            create_commit.dispatch(());
        }>
            <input type="text" node_ref=title_ref/>
            <textarea node_ref=body_ref></textarea>

            <input type="submit" value="create" />
        </form>
    }
}

#[server] 
pub async fn insert_commit(title: String, body: String, parent_id: Option<String>, tags: Option<Vec<String>>) -> Result<String, ServerFnError> {
    use axum_login::AuthSession;
    use crate::{
        app::AppState,
        auth::Backend,
        commit::Commit,
    };
    use leptos_axum::{extract, redirect};

    let tags = tags.unwrap_or_else(|| vec![]);

    let user_id = extract::<AuthSession<Backend>>().await?
        .user
        .ok_or(ServerFnError::new("Unauthorized"))?
        .user_id;

    let new_commit = Commit::build(
        None,
        user_id,
        parent_id,
        title,
        Some(body),
        tags,
        None,
    ).map_err(ServerFnError::new)?;

    let commit_service = use_context::<AppState>()
        .ok_or(ServerFnError::new("missing appstate"))?
        .db
        .commit_service;

    let id = commit_service.insert_one(new_commit).await
        .map_err(ServerFnError::new)
        .map(|i| i.inserted_id.to_string())?;

    redirect("/");
    Ok(id)
}