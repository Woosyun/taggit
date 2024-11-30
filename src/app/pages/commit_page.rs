use leptos::prelude::*;
use leptos::html;
use leptos_router::{
    hooks::use_params,
    params::Params,
};
use crate::{
    commit::Commit,
    app::{fetch_commit, insert_commit},
};

#[derive(Params, PartialEq, Clone)]
struct ContactParams {
    id: String
}

#[component] 
pub fn CommitPage() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let commit_id = move || params.get()
        .map(|p| p.id)
        .unwrap_or_else(|_| "".to_string());
    let commit = Resource::new(commit_id, |commit_id| async move {
        fetch_commit(commit_id).await
    });

    let title_ref: NodeRef<html::Input> = NodeRef::new();
    let body_ref: NodeRef<html::Textarea> = NodeRef::new();

    let create_branch = Action::new(move |input: &Commit| {
        let commit = input.to_owned();
        let title = title_ref.get().expect("missing title_ref").value();
        let body = body_ref.get().expect("missing body_ref").value();

        async move {
            insert_commit(title, body, commit.id, Some(commit.tags)).await
                .map(|re| dbg!(re))
        }
    });
    
    view! {
        <Suspense fallback=move || view! {"fetching commit..."}>
        <ErrorBoundary fallback=move |_| view! {<h1>"error while fetching commit"</h1>}>
            {move || {
                commit.get().map(|re| {
                    re.map(|commit| {
                        let title = commit.title.clone();
                        let body = commit.body.clone();
                        view! {
                            <form class="container" on:submit=move |ev| {
                                ev.prevent_default();
                                create_branch.dispatch(commit.clone());
                            }>
                                <input type="text" value=title node_ref=title_ref/>
                                <textarea node_ref=body_ref>{body}</textarea>
                                <input type="submit" value="commit" />
                            </form>
                        }
                    })
                })
            }}
        </ErrorBoundary>
        </Suspense>
    }
}