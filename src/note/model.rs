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

impl IntoRender for Note {
    type Output = Vec<AnyView>;

    fn into_render(self) -> Self::Output {
        let node_tree = vec![NodeRef::<Div>::new(); self.body.len()];

        let tmp= node_tree.clone();
        Effect::new(move || {
            let mut iter = tmp.iter();
            let mut prev_node: Option<&NodeRef::<Div>> = None;

            while let Some(cur_node) = iter.next() {
                if prev_node.is_none() { 
                    prev_node = Some(cur_node);
                    continue; 
                }

                let on_click: Closure<dyn Fn()> = Closure::new(|| {
                    let value = cur_node.get().expect("cannot get cur_node")
                        .inner_text();
                    log!("value under this element: {}", value);
                });
                prev_node.unwrap().get().expect("cannot get prev_node")
                    .set_onclick(Some(on_click.as_ref().unchecked_ref()));
            }
        });
        
        view! {
            {self.body.into_iter().zip(node_tree)
                .map(|(elem, node_ref)| elem.render(node_ref)).collect_view()
            }
        }
        //self.body.into_iter().map(IntoRender::into_render).collect_view()
    }
}