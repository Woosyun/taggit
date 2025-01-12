use serde::{Serialize, Deserialize};
use leptos::prelude::*;
use leptos::logging::log;
use leptos::html::Div;
//use wasm_bindgen::{closure::Closure, JsCast};

#[derive(Serialize, Deserialize)]
pub enum Element {
    H1(String),
    P(String)
}

impl IntoRender for Element {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        let node_ref = NodeRef::<Div>::new();
        //Effect::new(move || {
            //let on_click: Closure<dyn Fn()> = Closure::new(|| log!("clicked"));
            //node_ref.get().expect("cannot get node_ref")
                //.set_onclick(Some(on_click.as_ref().unchecked_ref()));
        //});
        let on_click = move |_| {
            let input = node_ref.get().expect("cannot get node_ref")
                .inner_text();
            log!("input: {}", input); 
        };

        view! {
            <div node_ref=node_ref on:click=on_click>
                {match &self {
                    Element::H1(content) => {
                        view! {
                            <h1>{content.clone()}</h1>
                        }.into_any()
                    },
                    Element::P(content) => {
                        view! {
                            <p>{content.clone()}</p>
                        }.into_any()
                    }
                }}
            </div>
        }.into_any()
    }
}