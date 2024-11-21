use leptos::prelude::*;
use leptos_router::{
    hooks::use_params,
    params::Params
};
use leptos::html;
use crate::note::Note;
use crate::comment::Comment;

#[derive(Clone, PartialEq, Params)]
struct ContactParams {
    id: String
}

#[component] 
fn ViewNote(note: Note) -> impl IntoView {
    view! {
        <p>"note id" {note.id}</p>
        <p>"note author id" {note.author_id}</p>
        <p>"note tags" {note.tags}</p>
        <p>"last modified" {note.last_modified}</p>
        <h1>{note.title}</h1>
        <div>{note.body}</div>
    }
}
#[component]
fn ViewComments(comments: Vec<Comment>) -> impl IntoView {
    comments.into_iter().map(|comment| view! {
        <ViewComment comment=comment />
    }).collect::<Vec<_>>()
}
#[component] 
pub fn ViewComment(comment: Comment) -> impl IntoView {
    view! {
        <div>
            <span>{comment.author_id}</span>
            {comment.body}
            <span>{comment.last_modified}</span>
        </div>
    }
}

#[component] 
pub fn ViewNotePage() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let id = move || match params.get() {
        Ok(p) => Some(p.id),
        _ => None
    };

    let note = Resource::new(id, |id| async move {
        fetch_note_by_id(id).await
    });

    let comments = Resource::new(id, |id| async move {
        fetch_comments(id).await
    });

    view! {
        <Suspense fallback=move || view! {<p>"loading note..."</p>} >
            <ErrorBoundary
                fallback=move |_| view! {<p>"error while fetching note"</p>}
            >
                {move || note.get().map(|note| {
                    note.map(move |note| view! {<ViewNote note=note />})
                })}
            </ErrorBoundary>
        </Suspense>

        // <NewComment note_id=id()/>

        <Suspense fallback=move || view! {<p>"loading comments..."</p>} >
            <ErrorBoundary fallback=move |_| view! {<p>"error while fetching comments"</p>}>
                {move || comments.get().map(|comments| {
                    comments.map(|comments| view! {<ViewComments comments=comments />})
                })}
            </ErrorBoundary>
        </Suspense>
    }
}

#[server(FetchCommentsByTargetId)] 
async fn fetch_comments(note_id: Option<String>) -> Result<Vec<Comment>, ServerFnError> {
    use crate::app::AppState;

    let note_id = note_id.ok_or(ServerFnError::new("missing note id"))?;
    
    let comment_service = use_context::<AppState>()
        .ok_or(ServerFnError::new("missing appstate"))?
        .db
        .comment_service;

    comment_service.find_by_target_id(note_id)
        .await
        .map_err(ServerFnError::new)
}

#[server(FetchNoteById)]
pub async fn fetch_note_by_id(id: Option<String>) -> Result<Note, ServerFnError> {
    use leptos::prelude::use_context;
    // use leptos_axum::redirect;
    use crate::app::AppState;
    
    let id = id.ok_or({
        // redirect("/");
        ServerFnError::new("note not found")
    })?;

    let note_service = use_context::<AppState>()
        .ok_or(ServerFnError::new("missing appstate"))?
        .db
        .note_service;

    note_service.find_one_by_id(id).await
        .map_err(ServerFnError::new)?
        .ok_or({
            // redirect("/");
            ServerFnError::new("note not found")
        })
}

#[component] 
pub fn NewComment(note_id: String) -> impl IntoView {
    let body_ref: NodeRef<html::Input> = NodeRef::new();
    let create_comment = Action::new(move |&()| {
        let note_id = note_id.clone();
        let body = body_ref.get().expect("missing body_ref").value();

        async move {
            insert_comment(note_id, body).await
        }
    });
    
    view! {
        <form on:submit = move |ev| {
            ev.prevent_default();
            create_comment.dispatch(());
        }>
            <input type="text" node_ref=body_ref/>
        </form>
    }
}

#[server(CreateComment)]
async fn insert_comment(note_id: String, body: String) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use axum_login::AuthSession;
    use crate::{
        auth::Backend,
        comment::Comment,
        app::AppState,
    };

    let comment_service = use_context::<AppState>()
        .ok_or(ServerFnError::new("app state error"))?
        .db
        .comment_service;
    
    let auth_session: AuthSession<Backend> = extract().await?;
    let user_id = auth_session.user
        .map(|user| user.user_id)
        .ok_or(ServerFnError::new("Unauthorized"))?;

    let new_comment = Comment::new(user_id, body, note_id, None);

    comment_service.insert_one(new_comment)
        .await 
        .map_err(|e| dbg!(e))
        .map_err(ServerFnError::new)?;
    
    Ok(())
}


