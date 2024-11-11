#![allow(unused)]

use leptos::prelude::{expect_context, server, use_context, ServerFnError};
use leptos::logging::log;

use serde::{Deserialize, Serialize};
use oauth2::{reqwest, PkceCodeVerifier};
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
use crate::models::config::Config;

pub enum OAuth {
    GOOGLE,
    // GITHUB,
}

impl OAuth {
    pub fn get_client_id_and_sceret(&self, config: Config) -> (String, String) {
        match self {
            OAuth::GOOGLE => (config.google_oauth_client_id, config.google_oauth_client_secret),
            // OAuth::GITHUB => (config.github_oauth_client_id, config.github_oauth_client_secret),
        }
    }
    pub fn get_auth_url(&self) -> String {
        match self {
            OAuth::GOOGLE => "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        }
    }
    pub fn get_token_url(&self) -> String {
        match self {
            OAuth::GOOGLE => "https://www.googleapis.com/oauth2/v3/token".to_string(),
        }
    }
    pub fn get_redirect_url(&self) -> String {
        match self {
            OAuth::GOOGLE => "http://localhost:3000/api/auth/callback/google".to_string(),
        }
    }
}

const CSRF_STATE: &str = "csrf_state";
const PKCE_CODE_VERIFIER: &str = "pkce_code_verifier";

#[server(Login, "/api")]
pub async fn try_login(provider: String) -> Result<(), ServerFnError> {
    use leptos_axum::{redirect, extract};
    use crate::models::config::AppState;
    use tower_sessions::Session;

    // let session = expect_context::<Session>();
    let (session): (Session) = extract().await?;

    let appstate = use_context::<AppState>()
        .unwrap();
    let config = appstate.config;

    let provider = OAuth::GOOGLE;
    let (client_id, client_secret) = provider.get_client_id_and_sceret(config);
    let auth_url = provider.get_auth_url();
    let token_url = provider.get_token_url();
    let redirect_url = provider.get_redirect_url();
    
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url)
            .expect("Invalid authorization endpoint URL"),
        Some(TokenUrl::new(token_url)
            .expect("Invalid token endpoint URL")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)
        .expect("redirect_url should be set"));

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    session.insert(CSRF_STATE, csrf_state).await.unwrap();
    session.insert(PKCE_CODE_VERIFIER, pkce_code_verifier).await.unwrap();

    
    // log!("session after inserting values: {session:#?}");
    
    // let csrf_state = session.get_value("csrf_state").await.unwrap().unwrap();
    // let pkce_code_verifier = session.get_value("pkce_code_verifier").await.unwrap().unwrap();
    // log!("csrf_state inserted in session: {csrf_state:#?}");
    // log!("pkce_code_verifier inserted in session: {pkce_code_verifier:#?}");

    
    redirect(&auth_url.to_string());
    
    Ok(())
}


use cfg_if::cfg_if;
cfg_if!{
    if #[cfg(feature="ssr")] {
        use leptos::prelude::*;
        use axum::{
            extract::{
                State, 
                Query
            },
            response::Response,
            http::StatusCode,
        };
        use tower_sessions::Session;
        use std::collections::HashMap;
        
        pub async fn handle_auth_callback(
            State(leptos_options): State<LeptosOptions>,
            Query(params): Query<HashMap<String, String>>,
            session: Session,
        ) -> Response {
            log!("extracted session: {session:#?}");
            
            let state = params.get("state").unwrap();
            let code = params.get("code").unwrap();
            
            let csrf_state = session.get::<String>(CSRF_STATE).await;
            log!("csrf_state: {csrf_state:#?}");

            // let csrf_state: String = session.get("csrf_state")
            //     .await
            //     .unwrap()
            //     .unwrap();
            // log!("csrf_state: {csrf_state:#?}");
            // let pkce_code_verifier: String = session.get("pkce_code_verifier").await.unwrap().unwrap();
            // log!("pkce_code_verifier: {pkce_code_verifier:#?}");
        
            Response::builder()
            .status(StatusCode::ACCEPTED)
            .body("authenticated!!".into())
            .unwrap()
        }


    }
}
