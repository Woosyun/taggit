use leptos::prelude::*;
use leptos_router::params::ParamsMap;

pub fn get_tags_from_query(q: Memo<ParamsMap>) -> Vec<String> {
    q
        .get()
        .get_all("tags")
        .map(|tags| {
            tags
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}