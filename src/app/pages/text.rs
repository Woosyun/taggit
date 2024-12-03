use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::{
    app::{
        utils::*,
        frontend::Authenticated,
    },
    text::{TextItem, Text, self},
};

#[component] 
pub fn Page() -> impl IntoView {
    let query = use_query_map();
    let tags = move || get_tags_from_query(&query.get());
    let authenticated = use_context::<Authenticated>().expect("missing authenticated context").0;
    
    let commit = Resource::new(query, |query| async move {
        let parent_id = get_parent_id_from_query(&query);
        let child_id = get_child_id_from_query(&query);

        fetch_commit(parent_id, child_id).await.unwrap_or_default()
    });

    let child_commits = Resource::new(query, |query| async move {
        let parent_id = get_parent_id_from_query(&query);

        fetch_child_commits(parent_id).await.unwrap_or_default()
    });
    
    view! {
        <Suspense fallback=move || view! {<h1>"fetching commit..."</h1>}>
            {move || Suspend::new(async move {
                let commit = commit.await;
                let authenticated = authenticated.await.is_ok();

                view! {
                    <text::view::Editor text=commit tags=tags() authenticated=authenticated/>
                }
            })}
        </Suspense>

        <Suspense fallback=move || view! {<p>"fetching child commits..."</p>}>
            {move || child_commits.get().map(|items| {
                items.into_iter().map(|item| {
                    view! {
                        <text::view::SearchItem item=item />
                    }
                }).collect_view()
            })}
        </Suspense>
    }
}

/*
    1. parent_id == NULL => commit (create repository)
    2. parent_id != NULL && child_id == NULL => commit (create branch)
    3. parent_id != NULL && child_id != NULL => diff
 */

#[server] 
async fn fetch_commit(parent_id: Option<String>, child_id: Option<String>) -> Result<Text, ServerFnError> {
    use crate::app::AppState;
    use leptos_axum::redirect;
    
    let parent_id = match parent_id {
        Some(id) => id,
        None => {
            return Ok(Text::default());
        }
    };
    let target_id = child_id.unwrap_or_else(|| parent_id);

    let text_service = use_context::<AppState>()
        .expect("missing AppState")
        .db
        .text_service;

    text_service.find_one_by_id(target_id).await
        .map_err(|e| {
            redirect("/");
            ServerFnError::new(dbg!(e))
        })?
        .ok_or_else(|| {
            redirect("/");
            ServerFnError::new(dbg!("invalid id. No text found"))
        })
}

/*
    1. parent_id == NULL => nothing
    2. parent_id != NULL => child commits
 */

#[server]
async fn fetch_child_commits(parent_id: Option<String>) -> Result<Vec<TextItem>, ServerFnError> {
    use crate::app::AppState;
    
    let parent_id = match parent_id {
        Some(id) => id,
        None => return Ok(vec![])
    };

    let text_service = use_context::<AppState>()
        .expect("missing AppState")
        .db
        .text_service;

    text_service.find_items_by_parent_id(parent_id).await
        .map_err(ServerFnError::new)
}