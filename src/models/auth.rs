#![allow(unused)]

use serde::{Serialize, Deserialize};
use crate::models::config::Config;

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
    url::Url,
    CsrfToken, 
    PkceCodeChallenge, 
    RedirectUrl,
    RevocationUrl, 
    Scope, 
    TokenUrl,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    user_email: String,
}

pub enum OAuth {
    GOOGLE,
    GITHUB,
}

impl OAuth {
    pub fn get_client_values(self, config: Config) -> Option<(String, String)> {
        match self {
            OAuth::GOOGLE => Some((config.google_oauth_client_id, config.google_oauth_client_secret)),
            OAuth::GITHUB => Some((config.github_oauth_client_id, config.github_oauth_client_secret)),
            _ => None
        }
    }

    pub fn start_with_google(config: Config) -> Option<(Url, CsrfToken)> {
        let client = BasicClient::new(
            ClientId::new(config.google_oauth_client_id),
            Some(ClientSecret::new(config.google_oauth_client_secret)),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("Invalid authorization endpoint URL"),
            Some(TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
                .expect("Invalid token endpoint URL")),
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000/api/auth/redirect/google".to_string())
            .expect("redirect_url should be set"));
        
            
        // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    
        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            // This example is requesting access to the "calendar" features and the user's profile.
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/plus.me".to_string(),
            ))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        Some((authorize_url, csrf_state))
    }
}