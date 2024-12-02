use leptos_router::params::ParamsMap;

pub fn get_tags_from_query(q: &ParamsMap) -> Vec<String> {
    q.get_all("tags")
        .map(|tags| {
            tags
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

pub fn get_parent_id_from_query(q: &ParamsMap) -> Option<String> {
    q.get("parent_id")
}

pub fn get_child_id_from_query(q: &ParamsMap) -> Option<String> {
    q.get("child_id")
}


use web_sys::window;
pub fn alert(str: &str) {
    window().unwrap().alert_with_message(str).unwrap();
}