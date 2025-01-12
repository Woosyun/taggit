pub mod app;
pub mod user;
pub mod text;
pub mod note;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature="ssr")] {
        pub mod db;
        pub mod auth;
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::Frontend);
}