use leptos::{html, prelude::*, ev, logging::log};
use leptos_router::{
    hooks::*,
    components::A,
};
use crate::{note::Note, user::PublicUserProfile};

#[component]
pub fn HomePage() -> impl IntoView {
    use web_sys::window;

    let query = use_query_map();
    let tags = move || query
        .get()
        .get_all("tags")
        .unwrap_or_else(|| vec![]);

    Effect::new(
        move |_| {
            log!("tags: {:?}", tags());
        }
    );
    
    let note_items = Resource::new( tags, |tags: Vec<String>| async move {
        search(tags).await
            .unwrap_or_else(|e| {
                window().unwrap().alert_with_message("(HomePage) something is wrong while searching").unwrap();
                window().unwrap().alert_with_message(&e.to_string()).unwrap();
                vec![]
            })
    });
    let note_items = move || {
        note_items
            .get()
            .unwrap_or_else(|| vec![])
    };

    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: ev::SubmitEvent| {
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

        log!("new query: {}", new_query);
        
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

        log!("new query: {}", new_query);
        
        window().unwrap().location().set_search(&new_query).unwrap();
    };

    // let url_for_new_note = move || format!("create/{}", query.get().to_query_string());
    let url_for_new_note = move || {
        let a = "create".to_string();
        let b = query.get().to_query_string();

        a+b.as_str()
    };

    let user = use_context::<PublicUserProfile>().unwrap();
    let authenticated = move || user.authenticated;
    let auth_button = move || match authenticated() {
        false => view! {<A href="/login">login</A>}.into_any(),
        true => view! {<button>logout</button>}.into_any(),
    };
    //TODO: use <Show />
    
    view! {
        <div class="topbar">
            <A href=url_for_new_note>+</A>

            <form on:submit=on_submit>
                <input type="search" node_ref=input_ref/>
                <input type="submit" value="search" />
            </form>
            
            {auth_button}
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
        
        <Transition fallback=move || view! { <p>"loading notes"</p>}>
            <ul>
                <For each=note_items key=|note| note.id.clone() children=move |note: Note| {
                    view! {
                        <li>
                            <a href=format!("/edit?id={}", note.id.unwrap())>
                                <h2>{note.title}</h2>
                            </a>
                        </li>
                    }
                } />
            </ul>
        </Transition>
    }
}



#[server(Search, "/api")]
pub async fn search(tags: Vec<String>) -> Result<Vec<Note>, ServerFnError> {
    use crate::app::AppState;
    use leptos::prelude::use_context;

    // let note_service = match use_context::<DB>() {
    //     Some(db) => db.note_service,
    //     None => return Err(ServerFnError::ServerError("cannot connect to database".to_string()))
    // };
    let note_service = use_context::<AppState>()
        .unwrap()
        .db
        .note_service;

    // log!("(api/search)searching notes by tags {:?}", tags);

    let note_items = note_service
        .find_items_by_tags(tags)
        .await
        .map_err(|err| ServerFnError::ServerError(err.to_string()));

    // {
    //     match &note_items {
    //         Ok(note_items) => {
    //             log!("(api/search)found note items: {:#?}", note_items);
    //         }
    //         Err(err) => {
    //             log!("(api/search)error while fetching note items: {:#?}", err);
    //         }
    //     }
    // }

    note_items
}


#[server(FetchNoteById, "/api")]
pub async fn fetch_note_by_id(id: String) -> Result<Note, ServerFnError> {
    //check if the note exists
    use leptos::prelude::use_context;
    use crate::app::AppState;
    
    // let note_service = match use_context::<DB>() {
    //     Some(db) => db.note_service,
    //     None => return Err(ServerFnError::ServerError("cannot connect to database".to_string()))
    // };
    let note_service = use_context::<AppState>()
        .unwrap()
        .db
        .note_service;

    match note_service.find_one(id).await {
        Ok(note_option) => {
            match note_option {
                Some(note) => Ok(note),
                None => Err(ServerFnError::ServerError("note not found".to_string()))
            }
        },
        Err(err) => Err(ServerFnError::ServerError(format!("error while fetching note: {:?}", err)))
    }
}