#![allow(unused)]

use leptos::prelude::*;
use serde::{Serialize, Deserialize};
use crate::note::Element;
use leptos::html::Div;
use wasm_bindgen::{closure::Closure, JsCast};
use leptos::logging::log;

#[derive(Serialize, Deserialize, Default)]
pub struct Note {
    pub title: String,
    pub id: Option<String>,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub last_modified: Option<String>,
    #[serde(default)]
    pub body: Vec<Element>, // maybe Vec<NodeRef<Div>>
    #[serde(default)]
    pub tags: Vec<String>,
}
impl Note {
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
    pub fn set_author_id(&mut self, id: String) {
        self.author_id = id;
    }
    pub fn update_last_modified(&mut self, date: String) {
        self.last_modified = Some(date);
    }
}

pub struct EventFunc<T: FnMut()> {
    pub on_click: T,
}

impl IntoRender for Note {
    type Output = Vec<AnyView>;

    fn into_render(self) -> Self::Output {
        // TODO: find out how to prevent Closure type being dropped.
        // 1. make it signal => X
        //    : trait bound issues
        // 2. put variable to broader scope (higher level, attribute of struct)
        // 3. make it static
        // 4. make it with type Closure_once_js(?)

        let node_tree = (0..self.body.len()).map(|_| NodeRef::<Div>::new()).collect::<Vec<_>>();
        let (node_tree, _) = signal(node_tree);

        //let ev_tree = (0..self.body.len()).map(|_| None).collect::<Vec<_>>();
        //let (ev_tree, set_ev_tree) = signal(ev_tree);

        Effect::new(move || {
            let mut prev_node: Option<NodeRef::<Div>> = None;
            //let mut ev_idx = 0;

            for node in node_tree.get() {
                if prev_node.is_some() {
                    let on_click: Closure<dyn Fn()> = Closure::new(move || {
                        let value = node.get().expect("cannot get current node")
                            .inner_text();
                        log!("value under this element: {}", value);
                    });

                    prev_node.unwrap().get().expect("cannot get prev_node")
                        .set_onclick(Some(on_click.as_ref().unchecked_ref()));

                    prev_node = Some(node);
                }
                prev_node = Some(node);
            }
        });
        
        view! {
            {
                self.body.into_iter()
                    .map(IntoRender::into_render)
                    .collect_view()
            }
        }
    }
}
