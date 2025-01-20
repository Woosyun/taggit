use leptos::prelude::*;
use serde::{Serialize, Deserialize};
use crate::note::Element;
use leptos::html::Div;
use wasm_bindgen::{closure::Closure, JsCast};
use leptos::logging::log;
use std::rc::Rc;

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

#[derive(Clone)]
pub struct DomNode {
    pub element: Element,
    pub node_ref: NodeRef<Div>,
    pub onclick: Rc<Option<Closure<dyn FnMut()>>>
}
impl DomNode {
    pub fn new(elem: Element) -> Self {
        Self {
            element: elem,
            node_ref: NodeRef::new(),
            onclick: Rc::new(None)
        }
    }

    pub fn set_onclick(&mut self, onclick: Closure<dyn FnMut()>) {
        self.onclick = Rc::new(Some(onclick));
        let onclick = self.onclick.as_ref()
            .as_ref().map(|c| c.as_ref().unchecked_ref());
        self.node_ref.get().expect("cannot get node_ref")
            .set_onclick(onclick);
    }
}
impl IntoRender for DomNode {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        view! {
            <div node_ref=self.node_ref>
                {self.element}
            </div>
        }.into_any()
    }
}

impl IntoRender for Note {
    type Output = Vec<AnyView>;

    fn into_render(self) -> Self::Output {
        let dom_nodes_ = self.body.into_iter().map(DomNode::new).collect::<Vec<_>>();
        let (dom_nodes, set_dom_nodes) = arc_signal(dom_nodes_);
        //TODO: current node may not respond(reactive) to change,
        //      so check whether effect runs with change of dom_nodes' contents.

        Effect::new(move || {
            set_dom_nodes.update(|nodes| {
                let mut prev_node: Option<&mut DomNode> = None;

                for node in nodes.iter_mut() {
                    if let Some(prev_node) = prev_node {
                        let onclick: Closure<dyn FnMut()> = {
                            let node_ref = node.node_ref.clone();
                            Closure::new(move || {
                                if let Some(node) = node_ref.get() {
                                    let value = node.inner_text();
                                    log!("value under this element: {}", value);
                                } else {
                                    log!("Failed to get node_ref");
                                }
                            })
                        };

                        prev_node.set_onclick(onclick);
                    }
                    prev_node = Some(node);
                }
            });
        });

        dom_nodes.get()
            .into_iter()
            .map(IntoRender::into_render)
            .collect_view()
    }
}
