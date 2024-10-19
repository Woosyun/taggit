use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
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
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    use leptos::logging::log;

    let (tags, set_tags) = create_signal(HashSet::<String>::new());

    let note_items = create_resource(tags, |tags| async move {
        log!("searching notes by tags {:?}", tags);

        api::search(tags)
            .await
            .unwrap_or_else(|err| {
                log!("error while fetching note items: {:?}", err);
                vec![]
            })
    });

    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();

        set_tags.update(|tags: &mut HashSet<String>| {
            tags.insert(value);
        });

        input_element()
            .expect("<input> to exist")
            .set_inner_text("");
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" placeholder="..." node_ref=input_element />
            <button type="submit">submit</button>
        </form>

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

        {move || match note_items.get() {
            Some(note_items) => {
                view! {
                    <ul>
                        {note_items.into_iter().map(move |note_item| {
                            view! {
                                <li>
                                    <p>{note_item.title}</p>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                }
            },
            _ => {
                view! {
                    <ul>
                        <li>
                            <p>loading...</p>
                        </li>
                    </ul>
                }
            }
        }}
    }
}
