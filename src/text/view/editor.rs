use leptos::prelude::*;
use crate::text::Text;

#[component] 
pub fn Editor(text: Text, tags: Vec<String>, authenticated: bool) -> impl IntoView {
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
                <div class="body">
                    {body}
                </div>
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
            when=move ||  authenticated && !readonly.get()
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

