#![allow(unused)]

use leptos::prelude::{server, use_context, ServerFnError};
use leptos::logging::log;

use serde::{Deserialize, Serialize};
use oauth2::reqwest;
use oauth2::{
    basic::BasicClient, 
    StandardRevocableToken, 
    TokenResponse
};
use oauth2::{
    AuthUrl,
    AuthorizationCode, 
    ClientId, 
    ClientSecret, 
    CsrfToken, 
    PkceCodeChallenge, 
    RedirectUrl,
    RevocationUrl, 
    Scope, 
    TokenUrl,
};


#[server(Login, "/api")]
pub async fn login(provider: String) -> Result<(), ServerFnError> {
    use leptos_axum::redirect;
    use crate::models::auth::OAuth;
    use crate::models::config::AppState;

    let appstate = use_context::<AppState>()
        .unwrap();
    let config = appstate.config;

    let (authorize_url, _) = OAuth::start_with_google(config).unwrap();

    // println!("Open this URL in your browser:\n{authorize_url}\n");
    redirect(&authorize_url.to_string());
    
    Ok(())
}

#[server(
    name=HandleRedirectGoogle,
    prefix="/api", 
    endpoint="auth/redirect/google",
)]
pub async fn handle_redirect_google() -> Result<(), ServerFnError> {
    use leptos_axum::{extract, redirect};
    use axum::extract::Query;
    use std::collections::HashMap;
    use leptos::logging::log;
    use crate::models::auth::OAuth;
    use crate::models::config::AppState;

    let appstate= use_context::<AppState>().unwrap();
    let config = appstate.config;
    let leptos_options = appstate.options;
    let (authorize_url, csrf_state) = OAuth::start_with_google(config).unwrap();

    //TODO: get state(csrf_state) and code(authorization_code)
    let Query(params): Query<HashMap<String, String>> = extract().await?;
    log!("params: {params:#?}");

    // let token_result =
    // client
    //     .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
    //     // Set the PKCE code verifier.
    //     .set_pkce_verifier(pkce_verifier)
    //     .request(http_client)?;
    
    redirect("/");

    Ok(())
}