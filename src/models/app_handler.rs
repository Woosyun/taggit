use axum::response::IntoResponse;
use axum::{http, body::Body, extract::State};
use leptos_axum::handle_server_fns_with_context;
use leptos::prelude::*;
use leptos::logging::log;
use crate::{
    app::*,
    models::config::AppState,
};

pub async fn server_fn_handler(
    State(appstate): State<AppState>,
    request: http::Request<Body>
) -> impl IntoResponse {
    log!("******* server_fn_handler ******");

    handle_server_fns_with_context(move || {
        provide_context(appstate.clone());
    }, request).await
}

pub async fn leptos_routes_handler(
    State(appstate): State<AppState>,
    req: http::Request<Body>
) -> http::Response<Body> {
    log!("******** leptos routes handler ********");
    log!("Request: {req:#?}");

    let handler = leptos_axum::render_app_to_stream_with_context(
        move || {
            provide_context(appstate.db.clone());
        },
        || view! { <App/> }
    );
    handler(req).await
}