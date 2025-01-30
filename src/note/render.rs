use leptos::prelude::*;
use crate::note::{Note, Element};
use leptos::{
    html::Div,
    ev::{MouseEvent, KeyboardEvent, InputEvent}
};

#[component]
pub fn RenderNote(note: Note) -> impl IntoView {
    let (nodes, set_nodes) = signal(
        note.body
            .into_iter()
            .map(|elem| (elem, NodeRef::<Div>::new()))
            .collect::<Vec<_>>()
    );

    let (focus, set_focus) = signal(0_usize);
    Effect::new(move || {
        let focus_index = focus.get();
        match nodes.read_untracked().get(focus_index) {
            Some(node) => {
                node.1
                    .get().expect("node_ref cannot be missing")
                    .focus().expect("focus should work(?)");
            },
            None => ()
        }
    });

    let on_click = move |ev: MouseEvent, index: usize| {
        ev.prevent_default();

        let content = nodes.get()
            .get(index).expect("node at the index cannot be missing")
            .1.get().expect("node_ref cannot be missing")
            .inner_html();
        leptos::logging::log!("index: {}, inner_html: {}", index, content);
        set_focus(index);
    };

    let on_keydown = move |ev: KeyboardEvent, index: usize| {
        match ev.key().as_str() {
            "Enter" => {
                ev.prevent_default();
                set_nodes.update(|nodes| {
                    nodes.insert(index + 1, (Element::default(), NodeRef::<Div>::new()))
                });
                set_focus(index + 1);
            },
            "ArrowDown" => {
                let max_len = nodes.read()
                    .len();
                if index < max_len {
                    set_focus(index + 1);
                }
            },
            "ArrowUp" => {
                if index > 0 {
                    set_focus(index - 1);
                }
            },
            _ => ()
        }
    };

    // 문제점 발견!!
    // 라인 추가가 안된다!! (혹은 추가는 되었는데 업데이트가 안된다?)

    let on_beforeinput = move |ev: InputEvent, index: usize| {
        leptos::logging::log!("input type: {}", ev.input_type());
        if ev.input_type() == "deleteContentBackward" {
            let content = nodes.read()
                .get(index).expect("node at the index cannot be missing")
                .1.get().expect("node_ref cannot be missing")
                .inner_html();
            if content.is_empty() || content == "<br>" {
                ev.prevent_default();
                set_focus(index - 1);
                set_nodes.update(|nodes| {
                    nodes.remove(index);
                });
            }
        }
    };

    view! {
        <h1>length of lines: {move || nodes.read().len()}</h1>
        <div class="note">
        {move || nodes
            .get()
            .into_iter().enumerate()
            .map(|(idx, (elem, node_ref))| {
                view! {
                    <div
                        contenteditable=true
                        class="box"
                        node_ref=node_ref
                        on:click=move |ev| on_click(ev, idx)
                        on:keydown=move |ev| on_keydown(ev, idx)
                        on:beforeinput=move |ev| on_beforeinput(ev, idx)
                    >
                        {elem.content}
                    </div>
                }.into_any()
            })
            .collect_view()}
        </div>
    }.into_any()
}

