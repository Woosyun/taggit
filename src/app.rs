use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::{
    models,
    pages::*,
};

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
                    <Route path="" view=home_page::HomePage/>
                    <Route path="edit" view=EditPage/>
                    <Route path="create" view=create_note_page::CreateNotePage/>
                </Routes>
            </main>
        </Router>
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