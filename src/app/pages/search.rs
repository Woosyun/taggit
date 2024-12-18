use leptos::{html, prelude::*};
use leptos_router::{
    hooks::*,
    components::A,
};
use crate::{
    text,
    app::{
        pages::auth::AuthButton,
        utils::*,
    },
};

#[component] 
pub fn SearchBar() -> impl IntoView {
    use web_sys::window;
    
    let query = use_query_map();
    let input_ref: NodeRef<html::Input> = NodeRef::new();

    let commit_page_url = move || {
        let a = get_text_edit_page_url(None);
        let b = query.get().to_query_string();

        a+b.as_str()
    };
    
    view! {
        <div class="topbar">
            <A href=commit_page_url>+</A>

            <form on:submit=move |ev| {
                ev.prevent_default();

                let input = input_ref.get().expect("missing input_ref").value();
        
                let mut query = query.get();
                if get_tags_from_query(&query).contains(&input){
                    window().unwrap().alert_with_message("input is already in query").unwrap();
                    return;
                }
        
                query.insert("tags", input);
                let query = query.to_query_string();
        
                window().unwrap().location().set_search(&query).unwrap();
            }>
                <input type="search" node_ref=input_ref/>
                <input type="submit" value="search" />
            </form>
            
            <AuthButton />
        </div>
    }
}

#[component] 
pub fn TagBar() -> impl IntoView {
    use web_sys::window;
    
    let query = use_query_map();
    let tags = move || get_tags_from_query(&query.get());
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

    view! {
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
    }
}

#[component] 
pub fn SearchResultViewer() -> impl IntoView {
    let query = use_query_map();
    let items = Resource::new(query, |query| async move {
        let tags = get_tags_from_query(&query);
        
        search(Some(tags)).await
    });
    
    view! {
        <Transition fallback=move || view! { <p>"searching commits..."</p>}>
        <ErrorBoundary fallback=move |_| view! {<h1>"error while searching"</h1>}>
        <ul>
            {move || {
                items.get().map(|re| {
                    re.map(|items| {
                        items.into_iter().map(|item| {
                            view! {
                                <li>
                                    <TextItemViewer item=item />
                                </li>
                            }
                        }).collect_view()
                    })
                })
            }}
        </ul>
        </ErrorBoundary>
        </Transition>
    }
}

#[component] 
pub fn TextItemViewer(item: text::Item) -> impl IntoView {
    view! {
        <A href=get_text_view_page_url(None, item.id.expect("missing item.id"))>
            {item.title}
        </A>
    }
}

#[server]
pub async fn search(tags: Option<Vec<String>>) -> Result<Vec<text::Item>, ServerFnError> {
    use crate::app::AppState;
    use leptos::prelude::use_context;

    let tags = tags
        .unwrap_or_default();
    
    let search_service = use_context::<AppState>()
        .unwrap()
        .db.text_service;
    
    search_service.find_items_by_tags(tags).await
        .map_err(ServerFnError::new)
}

#[component] 
pub fn Page() -> impl IntoView {
    view! {
        <SearchBar />
        <TagBar />
        <SearchResultViewer />
    }
}

