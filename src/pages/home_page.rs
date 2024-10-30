use leptos::*;
use leptos_router::*;
use crate::{
    api, 
    models::Note,
};

//leptos 0.7버전 나오면 Vec<String>으로 바꿔서 사용하기
#[derive(Params, PartialEq, Clone, Debug)]
struct ContactQuery {
    tags: String
}

impl ContactQuery {
    fn default() -> Self {
        ContactQuery {
            tags: "".to_string()
        }
    }

    //TODO for 0.7 version: return self.tags
    fn get_tags(&self) -> Vec<String> {
        self.tags.split(',').map(|t| t.to_string()).collect()
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    use leptos::logging::log;
    use web_sys::window;

    let query = use_query::<ContactQuery>();
    let tags = move || {
        query.get()
            .unwrap_or_else(|_| ContactQuery::default())
            .get_tags()
    };

    let note_items = create_resource( tags, |tags: Vec<String>| async move {
        // let tags = tags.iter().map(|&t| t.to_string()).collect();
        api::search(tags).await
            .unwrap_or_else(|e| {
                window().unwrap().alert_with_message("(HomePage) something is wrong while searching").unwrap();
                window().unwrap().alert_with_message(&e.to_string()).unwrap();
                vec![]
            })
    });
    let note_items = move || {
        note_items
            .get()
            .unwrap_or_else(|| {
                log!("nothing returned");
                vec![]
            })
    };

    // let input_element: NodeRef<html::Input> = create_node_ref();
    // let on_submit = move |ev: ev::SubmitEvent| {
    //     ev.prevent_default();

    //     let value = input_element().expect("<input> to exist").value();

    //     //add to query.tags
    //     //redirect to new search-result page
        
    //     log!("input value: {value}");
    //     log!("existing tags: {:?}", tags());

    //     input_element()
    //         .expect("<input> to exist")
    //         .set_value("");
    // };

    view! {
        // <div id="topbar-container">
        //     <a href="create">+</a>
        
        //     <form on:submit=on_submit>
        //         <input type="text" placeholder="..." node_ref=input_element />
        //         // <button type="submit">submit</button>
                
        //     </form>

        //     <a href="login">login</a>
        // </div>
        <div id="topbar-container">
            <a href="create">+</a>

            <Form method="GET" action="">
                <input type="search" name="tags[]" />
                <For each=tags key=|tag| tag.clone() children=move |tag| {
                    view! {
                        <input type="search" name="tags[]" readonly="true" value=tag />
                    }
                } />
                <input type="submit" />
            </Form>

            <a href="login">login</a>
        </div>

        // {move || {
        //     query().to_vec()
        //         .iter()
        //         .map(|tag: &String| {
        //             let tmp = tag.clone();
        //             view! {
        //                 <span
        //                     class="badge"
        //                     on:click=move |_| {
        //                         set_tags.update(|tags: &mut HashSet<String>| {
        //                             tags.remove(&tmp);
        //                         })
        //                     }
        //                 >
        //                     {tag}
        //                 </span>
        //             }
        //         })
        //         .collect::<Vec<_>>()
        // }}

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