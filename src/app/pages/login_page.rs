#![allow(unused)]

use leptos::prelude::*;
use leptos::{logging::log, html::Input, ev};
use web_sys::{SubmitEvent, window};


#[component]
pub fn LoginPage() -> impl IntoView {
    let id_ref: NodeRef<Input> = NodeRef::new();
    let pw_ref: NodeRef<Input> = NodeRef::new();

    let login = Action::new(move |input: &()| {
        let id = id_ref.get().expect("id_ref missed!").value();
        let pw = pw_ref.get().expect("pw_ref missed!").value();
        // let id = input.0.clone();
        // let pw = input.1.clone();

        async move {
            login(id, pw).await
                .map_err(|e| {
                    window().unwrap().alert_with_message(&e.to_string());
                    e
                })
        }
    });
    let login = move |ev: SubmitEvent| {
        ev.prevent_default();
        login.dispatch(());
    };
    
    view! {
        <form on:submit=login>
            <label>
                "user id"
                <input type="text" node_ref=id_ref />
            </label>
            <label>
                "password"
                <input type="text" node_ref=pw_ref />
            </label>
            <input type="submit" />
        </form>
    }
}

#[server(Login)]
pub async fn login(id: String, password: String) -> Result<(), ServerFnError> {
    use leptos_axum::{extract, redirect};
    use axum_login::AuthSession;
    use crate::auth::{Backend, Credentials, SessionUser};
    use leptos::logging::log;

    let (mut auth_session): (AuthSession<Backend>) = extract().await?;

    let credentials= Credentials::new(id, password)
        .map_err(ServerFnError::new)?;

    log!("credentials: {credentials:?}");
    
    let user: SessionUser = match auth_session.authenticate(credentials).await {
        Ok(Some(user)) => Ok(user),
        // Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Ok(None) => Err(ServerFnError::new("UNAUTHORIZED".to_string())),
        Err(e) => Err(ServerFnError::new(e)),
    }?;

    // if auth_session.login(&user).await.is_err() {
    //     return Err(ServerFnError::ServerError("cannot login".to_string()));
    // }

    auth_session.login(&user).await.map_err(ServerFnError::new)?;

    redirect("/");
    
    Ok(())
}

#[server(Logout)] 
pub async fn logout() -> Result<(), ServerFnError> {
    use axum_login::AuthSession;
    use leptos_axum::extract;
    use crate::auth::Backend;

    let (mut auth_session): (AuthSession<Backend>) = extract().await?;

    auth_session.logout().await.map_err(ServerFnError::new)?;
    
    Ok(())
}