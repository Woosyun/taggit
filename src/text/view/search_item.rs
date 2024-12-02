use leptos::prelude::*;
use leptos_router::components::A;
use crate::text::Text;

#[component] 
pub fn SearchItem(text: Text) -> impl IntoView {
    let text_view_url = move || {
        format!("/text?parent_id={}", text.id.clone().expect("missing text id"))
    };

    view! {
        <A href=text_view_url>
            {text.title}
        </A>
    }
}