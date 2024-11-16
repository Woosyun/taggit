#![allow(unused)]

use leptos::prelude::*;
use leptos::{logging::log, html::Input, ev};
use web_sys::{SubmitEvent, window};
use crate::user::User;

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
        <a href="/register">register</a>
    }
}

#[server(Authenticate)]
pub async fn authenticate() -> Result<(), ServerFnError> {
    use leptos::logging::log;
    use leptos_axum::extract;
    use axum_login::AuthSession;
    use crate::auth::Backend;

    let (mut auth_session): (AuthSession<Backend>) = extract().await?;
    let user = auth_session.user;

    log!("(authenticate) user: {user:#?}");

    match user {
        Some(_) => Ok(()),
        None => Err(ServerFnError::ServerError("UNAUTHORIZED".to_string())),
    }
}

#[server(Login)]
pub async fn login(id: String, password: String) -> Result<(), ServerFnError> {
    use leptos_axum::{extract, redirect};
    use axum_login::AuthSession;
    use crate::auth::{Backend, Credentials};
    use leptos::logging::log;

    let (mut auth_session): (AuthSession<Backend>) = extract().await?;

    // make sure user logged out
    let user = &auth_session.user;
    if !user.is_none() {
        return Err(ServerFnError::ServerError("you have to logout to login!".to_string()));
    }

    // find user from db
    let credentials= Credentials::new(id, password)
        .map_err(ServerFnError::new)?;
    let user = match auth_session.authenticate(credentials).await {
        Ok(Some(user)) =>   Ok(user),
        // Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Ok(None) => Err(ServerFnError::new("UNAUTHORIZED".to_string())),
        Err(e) => Err(ServerFnError::new(e)),
    }?;

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

#[component]
pub fn RegisterPage() -> impl IntoView {
    use web_sys::window;

    let id_ref: NodeRef<Input> = NodeRef::new();
    let pw_ref: NodeRef<Input> = NodeRef::new();
    let name_ref: NodeRef<Input> = NodeRef::new();

    let register = Action::new(move |input: &()| {
        let id = id_ref.get().expect("missing id_ref").value();
        let pw = pw_ref.get().expect("missing pw_ref").value();
        let name = name_ref.get().expect("missing name_ref").value();

        async move {
            register(id, pw, name).await
                .map_err(|e| {
                    window().unwrap().alert_with_message(&e.to_string());
                    e
                })
        }
    });
    let register = move |ev: SubmitEvent| {
        ev.prevent_default();
        register.dispatch(());
    };
    
    view! {
        <form on:submit=register>
            <label>
                "user id"
                <input type="text" node_ref=id_ref />
            </label>
            <label>
                "password"
                <input type="text" node_ref=pw_ref />
            </label>
            <label>
                "user name"
                <input type="text" node_ref=name_ref />
            </label>
            <input type="submit" value="register"/>
        </form>
    }
}

#[server(Register)]
pub async fn register(user_id: String, password: String, user_name: String) -> Result<(), ServerFnError> {
    use axum_login::AuthSession;
    use leptos_axum::{extract, redirect};
    use crate::auth::Backend;

    let user = User::new(user_id, password, user_name);
    
    let (mut auth_session): (AuthSession<Backend>) = extract().await?;
    let backend = auth_session.backend;
    backend.register(user).await
        .map_err(ServerFnError::new)?;
    
    redirect("/");
    Ok(())
}