use leptos::{html, prelude::*, ev, logging::log};
use leptos_router::{
    hooks::*,
    components::A,
};
use crate::{note::Note, app::auth::AuthButton};

#[component]
pub fn HomePage() -> impl IntoView {
    use web_sys::window;

    let query = use_query_map();
    let tags = move || query
        .get()
        .get_all("tags")
        .map(|tags| {
            tags
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    Effect::new(
        move |_| {
            log!("tags: {:?}", tags());
        }
    );
    
    let note_items = Resource::new( tags, |tags: Vec<String>| async move {
        search(tags).await
            .unwrap_or_default()
    });
    let note_items = move || {
        note_items
            .get()
            .unwrap_or_default()
    };

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let add_tag = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let input = input_ref.get().expect("input ref to exists").value();

        //check validity of input
        if tags().contains(&input) {
            window().unwrap().alert_with_message("input is already in query").unwrap();
            return;
        }

        let mut new_query = query
            .get();
        new_query.insert("tags", input);
        let new_query = new_query.to_query_string();

        window().unwrap().location().set_search(&new_query).unwrap();
    };
    let delete_tag = move |tag: String| {
        if !tags().contains(&tag) {
            return;
        }

        let mut new_tags = tags();
        new_tags.retain(|t| t != &tag);
        let mut new_query = query
            .get();

        new_query.remove("tags");
        for tag in new_tags {
            new_query.insert("tags", tag);
        }
        let new_query = new_query.to_query_string();
        
        window().unwrap().location().set_search(&new_query).unwrap();
    };

    // let url_for_new_note = move || format!("create/{}", query.get().to_query_string());
    let url_for_new_note = move || {
        let a = "create".to_string();
        let b = query.get().to_query_string();

        a+b.as_str()
    };
    let url_for_note_view = move |note_id: &str| {
        let target_url = "/view/note/".to_string();
        target_url+note_id
    };
    
    view! {
        <div class="topbar">
            <A href=url_for_new_note>+</A>

            <form on:submit=add_tag>
                <input type="search" node_ref=input_ref/>
                <input type="submit" value="search" />
            </form>
            
            {AuthButton}
        </div>

        <div class="tagbar">
            <For each=tags key=|tag| tag.clone() children=move |tag: String| {
                let tag0 = tag.clone();
                view! {
                    <span class="badge" on:click=move |ev| {
                        ev.prevent_default();
                        delete_tag(tag0.clone());
                    }>{tag}</span>
                }
            } />
        </div>
        
        <Transition fallback=move || view! { <p>"loading notes..."</p>}>
            <ul>
                <For each=note_items key=|note| note.id.clone() children=move |note: Note| {
                    let id = note.id.unwrap();
                    view! {
                        <li>
                            <A href=move || url_for_note_view(&id.clone())>
                                <h2>{note.title}</h2>
                            </A>
                        </li>
                    }
                } />
            </ul>
        </Transition>
    }
}

#[server(Search)]
pub async fn search(tags: Vec<String>) -> Result<Vec<Note>, ServerFnError> {
    use crate::app::AppState;
    use leptos::prelude::use_context;

    let note_service = use_context::<AppState>()
        .unwrap()
        .db
        .note_service;

    let note_items = note_service
        .find_items_by_tags(tags)
        .await
        .map_err(ServerFnError::new);

    note_items
}
