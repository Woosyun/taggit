use leptos::prelude::*;
use leptos_router::{
    hooks::{use_query_map, use_params},
    params::Params,
};
use crate::{
    app::{
        utils::get_tags_from_query,
        Authenticated
    },
    commit::CommitEditor,
};

#[derive(Clone, PartialEq, Params)]
struct ContactParams {
    id: String,
}

#[component]
pub fn CommitPage() -> impl IntoView {
    let id = move || use_params::<ContactParams>()
        .get()
        .map(|p| Some(dbg!(p.id)))
        .map_err(|e| dbg!(e))
        .unwrap_or_else(|_| None);
        // .expect("id field should exist!");
    let tags = move || get_tags_from_query(use_query_map());
    
    let authenticated = use_context::<Authenticated>().expect("missing authenticated value").0;
    //since we don't want to page goes error even if user isn't logged in
    let authenticated = move || authenticated.get()
        .map(|re| dbg!(re).is_ok());
    
    view! {
        <Suspense fallback=move || view! {"authorizing..."} >
            {move || authenticated().map(|authenticated| view! {<CommitEditor commit_id=id tags=tags authenticated=authenticated/>})}
        </Suspense>
    }
}