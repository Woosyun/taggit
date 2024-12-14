use leptos::prelude::*;
use crate::text::{self, Text};

#[component] 
pub fn CommitViewer(parent: Option<Text>, child: Option<Text>) -> impl IntoView {
    let before = parent.as_ref().expect("missing parent")
        .body.as_ref()
        .expect("missing body")
        .lines()
        .collect::<Vec<&str>>();
    let after = child.as_ref().expect("missing child")
        .body.as_ref()
        .expect("missing body")
        .lines()
        .collect::<Vec<&str>>();

    let (len_of_lcs, edit_script) = text::myers_diff(0, &before, &after).expect("failed to diff");
    let len_of_add = after.len() - len_of_lcs;
    let len_of_delete = before.len() - len_of_lcs;
    let body = text::apply_edit_actions(before, edit_script).expect("failed to apply commit")
        .into_iter()
        .map(|str| {
            view! {
                <p>{str}</p>
            }
        })
        .collect::<Vec<_>>();

    
    view! {
        <h1>"numer of deletion: "{len_of_delete}</h1>
        <h1>"numer of addition: "{len_of_add}</h1>

        <div>
            {body}
        </div>
    }
}