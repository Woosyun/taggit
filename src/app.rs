use crate::error_template::{AppError, ErrorTemplate};
use ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use models::Note;
use std::collections::HashSet;

pub mod api;
pub mod models;
#[cfg(feature = "ssr")]
pub mod db;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="pkg/taggit.css"/>
        // <Stylesheet id="leptos" href="pkg/taildwind.css"/>
        <Title text="Welcome to Leptos"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="edit" view=EditPage/>
                    <Route path="create" view=CreateNotePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[allow(unused_imports, unused_variables)]
#[component]
fn HomePage() -> impl IntoView {
    use leptos::logging::log;
    use web_sys::window;

    let (tags, set_tags) = create_signal(HashSet::<String>::new());

    let note_items = create_resource(tags, |tags| async move {
        match api::search(tags).await {
            Ok(note_items) => note_items,
            Err(err) => {
                window().unwrap().alert_with_message("(HomePage) something is wrong while searching").unwrap();
                window().unwrap().alert_with_message(&err.to_string()).unwrap();
                vec![]
            }
        }
    });
    let note_items = move || {
        note_items
            .get()
            .unwrap_or_else(|| vec![])
    };

    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();

        set_tags.update(|tags: &mut HashSet<String>| {
            tags.insert(value);
        });

        input_element()
            .expect("<input> to exist")
            .set_value("");
    };

    view! {
        <div id="topbar-container">
            <a href="create">+</a>
        
            <form on:submit=on_submit>
                <input type="text" placeholder="..." node_ref=input_element />
                // <button type="submit">submit</button>
            </form>

            <a href="login">login</a>
        </div>

        {move || {
            tags()
                .iter()
                .map(|tag: &String| {
                    let tmp = tag.clone();
                    view! {
                        <span
                            class="badge"
                            on:click=move |_| {
                                set_tags.update(|tags: &mut HashSet<String>| {
                                    tags.remove(&tmp);
                                })
                            }
                        >
                            {tag}
                        </span>
                    }
                })
                .collect::<Vec<_>>()
        }}

        <Transition fallback=move || view! { <p>"loading notes"</p>}>
            <ul>
                <For each=note_items key=|note| note._id.clone() children=move |note: Note| {
                    view! {
                        <li>
                            <a href=format!("/edit?id={}", note._id.unwrap())>
                                <h2>{note.title}</h2>
                            </a>
                        </li>
                    }
                } />
            </ul>
        </Transition>
    }
}

#[allow(unused_variables, unused_imports)]
#[component]
fn CreateNotePage() -> impl IntoView {
    use logging::log;
    use web_sys::window;
    let window = window().unwrap();

    let title_ref = create_node_ref::<html::Input>();
    let body_ref = create_node_ref::<html::Textarea>();

    let insert_note = create_action(|input: &(String, String, Vec<String>, String)| {
        let title = input.0.to_owned();
        let body = input.1.to_owned();
        let tags = input.2.to_owned();
        let author_id = input.3.to_owned();

        log!("(CreateNotePage) inserting note with title: {}, body: {}, tags: {:?}, author_id: {:?}.", &title, &body, &tags, &author_id);
        
        async move {
            api::insert_note(title, body, tags, author_id).await
        }
    });
    
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let title = title_ref().expect("title_ref to exist").value();
        if let Err(err) = Note::validate_title(&title) {
            window.alert_with_message(err).unwrap();
            return;
        }

        let body = body_ref().expect("body_ref to exist").value();
        if let Err(err) = Note::validate_body(&body) {
            window.alert_with_message(err).unwrap();
            return;
        }

        let tags: Vec<String> = vec![];
        let author_id = "admin".to_string();

        insert_note.dispatch((title, body, tags, author_id));

        ()
    };
    
    view! {
        <form class="note-container" on:submit=on_submit>
            <input type="text" placeholder="title" node_ref=title_ref/>
            <textarea class="note-body" node_ref=body_ref></textarea>
            <button type="submit">submit</button>
        </form>
    }
}


#[derive(Params, PartialEq, Clone, Debug)]
struct ContactNoteQuery {
    id: String,
}

#[allow(unused_variables)]
#[component]
fn EditPage() -> impl IntoView {
    let query = use_query::<ContactNoteQuery>();
    let (editor_status, set_editor_status) = create_signal(models::EditorStatus::new());
    let read_only = move || editor_status.with(|status| status.is_read_only());
    // let note = create_resource(query, |query| async move {
    //     match query {
    //         Ok(query) => {
    //             let id = query.id;
    //             if id.is_empty() {
    //                 // create default note
    //                 Ok(models::Note::default())
    //             } else {
    //                 match api::fetch_note_by_id(id).await {
    //                     Ok(note) => Ok(note),
    //                     Err(err) => Err(err.to_string())
    //                 }
    //             }
    //         }
    //         Err(err) => Err(err.to_string())
    //     }
    // });

    //if note is not found, return 404
    

    view! {
        <form class="note-container">
            <input type="text" placeholder="title" readonly=read_only />
            <textarea placeholder="body" readonly=read_only />
            <button type="submit">submit</button>
        </form>
        // <Transition fallback=>
        //     {move || note.get()}
        // </Transition>
    }
}