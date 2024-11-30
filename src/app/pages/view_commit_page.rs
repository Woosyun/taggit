use leptos::prelude::*;
use leptos_router::{
    hooks::use_params,
    params::Params,
    components::A,
};
use crate::commit::Commit;

#[derive(Params, PartialEq, Clone)]
struct ContactParams {
    id: String,
}

#[component] 
pub fn ViewCommitPage() -> impl IntoView {
    let params = use_params::<ContactParams>();
    let commit_id = move || params.get()
        .map(|p| p.id)
        .unwrap_or_else(|_| "".to_string());

    let commit = Resource::new(commit_id, |commit_id| async move {
        fetch_commit(commit_id).await
    });

    let commit_page_url = move || {
        let a = "/commit/".to_string();
        let b = commit_id();
        a+&b
    };

    view! {
        <Suspense fallback=move || view! {"fetching commit..."}>
        <ErrorBoundary fallback=move |_| view! {<h1>"error while fetching commit"</h1>}>
            {move || {
                commit.get().map(|re| {
                    re.map(|commit| {
                        view! {
                            <div class="container">
                                <input type="text" value=commit.title readonly=true />
                                <textarea readonly=true>{commit.body}</textarea>
                                <A href=commit_page_url>"commit"</A>
                            </div>
                        }
                    })
                })
            }}
        </ErrorBoundary>
        </Suspense>
    }
}

#[server] 
pub async fn fetch_commit(commit_id: String) -> Result<Commit, ServerFnError> {
    use crate::app::AppState;

    let commit_service = use_context::<AppState>()
        .ok_or(ServerFnError::new("missing appstate"))?
        .db
        .commit_service;

    commit_service.find_one_by_id(commit_id).await
        .map_err(ServerFnError::new)?
        .ok_or(ServerFnError::new("invalid commit id"))
}