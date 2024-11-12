#![allow(unused)]

use leptos::prelude::*;
use leptos::ev;
use leptos::logging::log;
use web_sys::window;

#[component]
pub fn LoginPage() -> impl IntoView {

    // log!("call try_login()");
    // spawn_local(async move {
    //     try_login("google".to_string())
    //         .await
    //         .unwrap_or_else(|e| log!("try_login return error: {e:#?}"));
    // });

    // let login = |provider: String| async move {
    //     try_login(provider).await.unwrap_or_else(|e| {
    //         window()
    //             .unwrap()
    //             .alert_with_message(&e.to_string())
    //             .unwrap()
    //     })
    // };
    let login_action = Action::new(|provider: &String| {
        let provider = provider.to_owned();
        async move {
            log!("click login button!!");
        }
    });
    let login = move |ev: ev::MouseEvent, provider: String| {
        ev.prevent_default();
        login_action.dispatch(provider);
    };

    view! {
        <div>
            <h1>Login</h1>
            <button on:click=move |ev| login(ev, "google".to_string())>google</button>
        </div>
    }
}
