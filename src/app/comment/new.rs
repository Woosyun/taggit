use leptos::prelude::*;

#[component] 
pub fn NewComment() -> impl IntoView {
    view! {
        <form>
            <input type="text" />
        </form>
    }
}