use leptos::prelude::*;
use leptos_router::{
    hooks::use_query_map,
    components::A,
};
use crate::{
    text::{Text, self},
    app::utils::{
        get_child_id_from_query,
        get_parent_id_from_query,
    }
};

#[component] 
pub fn Page() -> impl IntoView {
    let query = use_query_map();

    let parent = Resource::new(query, |query| async move {
        let parent_id = get_parent_id_from_query(&query);
        
        fetch_parent_revision(parent_id).await.expect("redirect didn't work")
    });
    // child != NULL always
    let child = Resource::new(query, |query| async move {
        let child_id = get_child_id_from_query(&query);
        
        fetch_child_revision(child_id).await.expect("redirect didn't work")
    });

    let children = Resource::new(query, |query| async move {
        let parent_id = get_parent_id_from_query(&query);

        fetch_children(parent_id).await.unwrap_or_default()
    });

    view! {
        <Suspense fallback=move || view! {<h1>"fetching revisions..."</h1>}>
            {move || Suspend::new(async move {
                let parent = parent.await;
                let child = child.await;

                view! {
                    <TextCommitViewer parent=parent child=child />
                }
            })}
        </Suspense>

        <Suspense fallback=move || view! {<p>"fetcing children..."</p>}>
            {move || children.get().map(|items| {
                items.into_iter().map(|item| {
                    view! {
                        <TextItemViewer item=item />
                    }
                }).collect_view()
            })}
        </Suspense>
    }
}

#[server] 
async fn fetch_parent_revision(parent_id: Option<String>) -> Result<Text, ServerFnError> {
    use crate::app::AppState;
    use leptos_axum::redirect;

    let text_service = use_context::<AppState>()
        .expect("missing AppState")
        .db
        .text_service;

    let id = match parent_id {
        Some(id) => id,
        None => return Ok(Text::default())
    };

    text_service.find_one_by_id(id).await
        .map_err(|e| {
            redirect("/");
            ServerFnError::new(e)
        })?
        .ok_or_else(|| {
            redirect("/");
            ServerFnError::new(dbg!("invalid id. No text found"))
        })
}

#[server] 
async fn fetch_child_revision(child_id: Option<String>) -> Result<Text, ServerFnError> {
    use crate::app::AppState;
    use leptos_axum::redirect;

    let text_service = use_context::<AppState>()
        .expect("missing AppState")
        .db
        .text_service;

    let id = child_id
        .ok_or_else(|| {
            redirect("/");
            ServerFnError::new("child_id should not be None")
        })?;

    text_service.find_one_by_id(id).await
        .map_err(|e| {
            redirect("/");
            ServerFnError::new(e)
        })?
        .ok_or_else(|| {
            redirect("/");
            ServerFnError::new(dbg!("invalid id. No text found"))
        })
}

#[server]
async fn fetch_children(parent_id: Option<String>) -> Result<Vec<text::Item>, ServerFnError> {
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

#[component] 
pub fn TextCommitViewer(parent: Text, child: Text) -> impl IntoView {
    let before = parent
        .body.as_ref().expect("missing body")
        .lines()
        .collect::<Vec<&str>>();
    let after = child
        .body.as_ref().expect("missing body")
        .lines()
        .collect::<Vec<&str>>();

    let (len_of_lcs, edit_script) = text::myers_diff(0, &before, &after).expect("failed to diff");
    let len_of_add = after.len() - len_of_lcs;
    let len_of_delete = before.len() - len_of_lcs;
    let body = text::apply_edit_actions(before, edit_script).expect("failed to apply commit")
        .into_iter()
        .map(|str| {
            view! {
                <p>{str}</p>
            }
        })
        .collect::<Vec<_>>();

    
    view! {
        <h1>"numer of deletion: "{len_of_delete}</h1>
        <h1>"numer of addition: "{len_of_add}</h1>

        <div>
            {body}
        </div>
    }
}

#[component] 
pub fn TextItemViewer(item: text::Item) -> impl IntoView {
    let text_view_url = move || {
        format!("/text/view?parent_id={}", item.id.clone().expect("missing text id"))
    };

    view! {
        <A href=text_view_url>
            {item.title}
        </A>
    }
}