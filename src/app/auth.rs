// #[server] 
// async fn authenticate() -> Result<(), ServerFnError> {
//     use leptos_axum::{extract, redirect};
//     use axum::http::StatusCode;
//     use axum_login::AuthSession;
//     use crate::auth::{Backend, Credentials};

//     let (auth_session, credentials): (AuthSession<Backend>, Credentials) = extract().await;

//     let user = match auth_session.authenticate(credentials.clone()).await {
//         Ok(Some(user)) => user,
//         // Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
//         Ok(None) => ServerFnError::ServerError("UNAUTHORIZED".to_string()),
//         Err(e) => ServerFnError::ServerError(e.to_string()),
//     };

//     if auth_session.login(&user).await.is_err() {
//         return ServerFnError::ServerError("cannot login".to_string());
//     }

//     redirect("/");
    
//     Ok(())
// }