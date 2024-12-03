use leptos::prelude::*;
use leptos_router::components::A;
use crate::text::TextItem;

#[component] 
pub fn SearchItem(item: TextItem) -> impl IntoView {
    let text_view_url = move || {
        format!("/text?parent_id={}", item.id.clone().expect("missing text id"))
    };

    view! {
        <A href=text_view_url>
            {item.title}
        </A>
    }
}