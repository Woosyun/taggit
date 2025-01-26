use serde::{Serialize, Deserialize};
use leptos::prelude::*;
use leptos::html::Div;
use leptos::ev::KeyboardEvent;

#[derive(Serialize, Deserialize, Default)]
pub struct Note {
    pub title: String,
    pub id: Option<String>,
    pub author_id: String,
    pub parent_id: Option<String>,
    pub last_modified: Option<String>,
    #[serde(default)]
    pub body: Vec<Element>,
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
impl IntoRender for Note {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
       let (nodes, set_nodes) = signal(
            self.body
                .into_iter()
                .map(|elem| {
                    (elem, NodeRef::<Div>::new())
                })
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

        let on_keydown = move |ev: KeyboardEvent, index: usize| {
            //leptos::logging::log!("keydown event: {:?}", ev.key());

            match ev.key().as_str() {
                "Enter" => {
                    ev.prevent_default();
                    set_nodes.update(|nodes| {
                        nodes.insert(index + 1, (Element::default(), NodeRef::<Div>::new()))
                    });

                    set_focus(index + 1);
                },
                "ArrowDown" => {
                    ev.prevent_default();
                    let focus = focus.get();
                    if focus < nodes.get_untracked().len() {
                        set_focus.set(focus + 1);
                    };
                },
                "ArrowUp" => {
                    ev.prevent_default();
                    let focus = focus.get();
                    if focus > 0 {
                        set_focus.set(focus - 1);
                    };
                },
                _ => ()
            }
        };
 
        view! {
            {move || nodes
                .get()
                .into_iter().enumerate()
                .map(|(idx, (elem, node_ref))| {
                    view! {
                        <div
                            contenteditable=true
                            node_ref=node_ref
                            on:keydown=move |ev| on_keydown(ev, idx)
                        >
                            {elem}
                        </div>
                    }.into_any()
                })
                .collect_view()}
        }.into_any()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Element {
    H1(String),
    P(String)
}

impl Default for Element {
    fn default() -> Self {
        Self::P("default".to_string())
    }
}

impl IntoRender for Element {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        match self {
            Element::H1(content) => {
                view! {
                    <h1>{content}</h1>
                }.into_any()
            },
            Element::P(content) => {
                view! {
                    <p>{content}</p>
                }.into_any()
            }
        }
    }
}

/*
use wasm_bindgen::{closure::Closure, JsCast};

#[derive(Clone)]
pub struct DomNode {
    pub element: Element,
    pub node_ref: NodeRef<Div>,
    pub onclick: Rc<Option<Closure<dyn FnMut()>>>
}

impl DomNode {
    pub fn set_onclick(&mut self, onclick: Closure<dyn FnMut()>) {
        self.onclick = Rc::new(Some(onclick));
        let onclick = self.onclick.as_ref()
            .as_ref().map(|c| c.as_ref().unchecked_ref());
        self.node_ref.get().expect("cannot get node_ref")
            .set_onclick(onclick);
    }
}
*/

/*
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
*/


