#![allow(unused)]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use axum::routing::post;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes, file_and_error_handler};
    use taggit::{
        app::*,
        db,
    };
    use time::Duration;
    use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
    use dotenv::dotenv;
    
    dotenv().ok();

    // let db_url = std::env::var("MONGODB_URI").expect("MONGODB_URI should exists");
    // let client = Client::with_uri_str(database_url).await?;
    // let session_store = MongoDBStore::new(client, "tower-sessions".to_string());
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let appstate = AppState {
        options: leptos_options.clone(),
        db: db::DB::new().await.unwrap(),
        // config: config::Config::new(),
    };
    // let db = db::DB::new().await.unwrap();

    // let app = Router::new()
    //     .route("/api/*fn_name", post(server_fn_handler))
    //     .leptos_routes_with_handler(routes, get(leptos_routes_handler))
    //     .fallback(file_and_error_handler::<AppState, _>(shell))
    //     .with_state(app_state);
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                // let db = db.clone();
                move || {
                    provide_context(appstate.clone());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            }
        )
        .layer(session_layer)
        .fallback(file_and_error_handler::<LeptosOptions, _>(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
