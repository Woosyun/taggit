use leptos::prelude::*;
use leptos::html;
use crate::commit::Commit;

//TOOD: change commit_id and tags parameter to receive literal values

#[component] 
pub fn CommitEditor<C, T>(commit_id: C, tags: T, authenticated: bool) -> impl IntoView 
where
    C: Fn() -> Option<String> + 'static + Send + Sync + Clone,
    T: Fn() -> Vec<String> + 'static + Send + Sync + Clone,
{
    use web_sys::window;

    let commit = Resource::new(commit_id.clone(), |commit_id| async move {
        fetch_commit(commit_id).await
    });

    let title_ref: NodeRef<html::Input> = NodeRef::new();
    let body_ref: NodeRef<html::Textarea> = NodeRef::new();

    let create_commit = Action::new(move |&()| {
        let title = title_ref.get().expect("missing title_ref").value();
        let body = body_ref.get().expect("missing body_ref").value();
        // let parent_id = commit_id();
        // let tags = Some(tags());
        
        let f = format!("title: {:?} body: {:?}", title, body);
        
        window().unwrap().alert_with_message(&f).unwrap();

        async move {
            insert_commit(title, body, commit_id(), Some(tags())).await
                .map_err(|e| {
                    dbg!(e)
                })
        }
    });

    let none_class = move || match authenticated {
        true => format!(""),
        false => format!("none")
    };

    view! {
        <Suspense fallback=move || view!{<h1>"waiting for loading commit..."</h1>} >
            <ErrorBoundary fallback=move |_| view! {<h1>"error while fetching commit: "</h1>}>
                {move || commit.get().map(|commit| {
                    commit.map(move |commit| view! {
                        <form class="container" on:submit=move |ev| {
                            ev.prevent_default();
                            create_commit.dispatch(());
                        }>
                            <input type="text" value=commit.title node_ref=title_ref readonly=!authenticated/>
                            <textarea node_ref=body_ref readonly=!authenticated >{commit.body}</textarea>

                            <input type="submit" value="commit" class=none_class />
                        </form>
                    })
                })}
            </ErrorBoundary>
        </Suspense>
    }
}

#[server] 
async fn fetch_commit(commit_id: Option<String>) -> Result<Commit, ServerFnError> {
    use crate::app::AppState;

    let commit_id = match commit_id {
        Some(id) => id,
        None => return Ok(Default::default())
    };

    let commit_service = use_context::<AppState>()
        .ok_or(ServerFnError::new("missing appstate"))?
        .db
        .commit_service;

    commit_service.find_one_by_id(commit_id).await
        .map_err(ServerFnError::new)?
        .ok_or(ServerFnError::new("no commit object in database"))
}

#[server] 
async fn insert_commit(title: String, body: String, parent_id: Option<String>, tags: Option<Vec<String>>) -> Result<String, ServerFnError> {
    use axum_login::AuthSession;
    use crate::{
        app::AppState,
        auth::Backend,
    };
    use leptos_axum::{extract, redirect};


    let tags = tags.unwrap_or_else(|| vec![]);

    let user_id = extract::<AuthSession<Backend>>().await?
        .user
        .ok_or(ServerFnError::new("Unauthorized"))?
        .user_id;

    dbg!(&user_id);

    let new_commit = Commit::build(
        None,
        user_id,
        parent_id,
        title,
        Some(body),
        tags,
        None,
    ).map_err(ServerFnError::new)?;

    dbg!(&new_commit);

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