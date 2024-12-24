use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use crate::{
    app::utils::*,
    text::Text,
};

/*
    parent == NULL && child == NULL if authenticated => commit (root)
    parent == NULL && child != NULL => diff (root)
    parent != NULL && child == NULL if authenticated => commit (non-root)
    parent != NULL && child != NULL => diff (non-root) + view child commits
*/

#[component] 
pub fn Page() -> impl IntoView {
    let query = use_query_map();
    // let _authenticated = use_context::<Authenticated>().expect("missing authenticated context").0;
    
    let parent_revision = Resource::new(query, |query| async move {
        let parent_id = get_parent_id_from_query(&query);

        fetch_revision(parent_id).await.expect("redirect didn't work")
    });
    
    view! {
        <Suspense fallback=move || view! {<h1>"fetching revision..."</h1>}>
        <ErrorBoundary fallback=move |_| view! {<h1>"failed to fetch revision"</h1>}>
            {move || parent_revision.get().map(|text| {
                let tags = get_tags_from_query(&query.get());
                
                view! {
                    <TextEditor text=text tags=tags />
                }
            })}
        </ErrorBoundary>
        </Suspense>
    }
}

#[server] 
async fn fetch_revision(id: Option<String>) -> Result<Text, ServerFnError> {
    use crate::app::AppState;
    use leptos_axum::redirect;

    //TODO: authenticate and redirect if user isn't exist

    let text_service = use_context::<AppState>()
        .expect("missing AppState")
        .db
        .text_service;

    let id = match id {
        Some(id) => id,
        None => return Ok(Text::default())
    };

    let text = text_service.find_one_by_id(id).await
        .map_err(|e| {
            redirect("/");
            ServerFnError::new(e)
        })?
        .ok_or_else(|| {
            redirect("/");
            ServerFnError::new(dbg!("invalid id. No text found"))
        })?;

    Ok(text)
}


#[component] 
pub fn TextEditor(text: Text, tags: Vec<String>) -> impl IntoView {
    let (title, set_title) = signal(text.title);
    let (body, set_body) = signal(text.body.unwrap_or_default());
    let (tags, _set_tags) = signal(tags);
    let (readonly, set_readonly) = signal(true);
    
    let commit_action = Action::new(move |&()| {
        let title = title.get();
        let body = body.get();
        let tags = tags.get();
        
        let text = Text::new(
            None,
            "".to_string(),
            text.id.clone(),
            title,
            Some(body),
            tags,
            None,
        );
        
        async move {
            commit(dbg!(text)).await
        }
    });

    let reader = move || {
        view! {
            <div class="container">
                <h1>{title}</h1>
                <textarea class="body" readonly>
                    {body}
                </textarea>
                <button class="btn" on:click=move |_| {
                    set_readonly(false);
                }>
                    "edit"
                </button>
            </div>
        }
    };

    view! {
        <Show
            when=move ||  !readonly.get()
            fallback=reader
        >
            <form class="container" on:submit=move |ev| {
                ev.prevent_default();
                commit_action.dispatch(());
            }>
                <input type="text" prop:value=title on:input:target=move |ev| {
                    set_title(ev.target().value());
                }/>
                <textarea class="body" prop:value=body on:input:target=move |ev| {
                    set_body(ev.target().value());
                }>{body}</textarea>
                <input type="submit" value="commit" class="btn"/>
            </form>
        </Show>
    }
}

#[server] 
async fn commit(text: Text) -> Result<String, ServerFnError> {
    use leptos_axum::{extract, redirect};
    use axum_login::AuthSession;
    use crate::{
        app::AppState,
        auth::Backend,
    };

    let user_id = extract::<AuthSession<Backend>>().await?
        .user
        .ok_or(ServerFnError::new("Unauthorized"))?
        .user_id;

    let text_service = use_context::<AppState>()
        .expect("missing AppState")
        .db.text_service;

    let mut text = text;
    text.set_author_id(user_id);

    let re = text_service.insert_one(dbg!(text)).await
        .map_err(ServerFnError::new)
        .map(|i| i.inserted_id.to_string());

    redirect("/");
    re
}
