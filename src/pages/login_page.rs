#![allow(unused)]

use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::api::auth::login;

#[component]
pub fn LoginPage() -> impl IntoView {
    use leptos::logging::log;
    // let appstate = use_context::<AppState>();
    // if !appstate.is_none() {
    //     log!("if this message showed, that means appstate is appeared in client side?");
    // }

    log!("call login()");
    spawn_local(async move {
        login("google".to_string())
            .await
        .unwrap_or_else(|e| log!("login return error: {e:#?}"));
    });
    
    view! {
        <h1>Login</h1>
        <a href="/api/auth/login">google</a>
        <a href="/">github</a>
    }
}