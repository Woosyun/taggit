use serde::{Serialize, Deserialize};
use leptos::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum Element {
    H1(String),
    P(String)
}

impl IntoRender for Element {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        match &self {
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
        }
    }
}
