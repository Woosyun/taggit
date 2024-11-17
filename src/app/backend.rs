#![allow(unused)]

use axum_macros::FromRef;
use leptos::prelude::*;
use leptos::logging::log;
use leptos_axum::{generate_route_list, LeptosRoutes, file_and_error_handler};
use crate::{
    db::DB,
    app::{Frontend, shell},
    auth::Backend,
};
use axum::Router;
use axum_login::{AuthManagerLayerBuilder, AuthManagerLayer};
use tower::{layer::util::{Identity, Stack}, ServiceBuilder};
use tower_sessions::{
    session_store::ExpiredDeletion,
    Expiry,
    SessionManagerLayer,
    SessionStore,
};
use tower_sessions_mongodb_store::{mongodb::Client, MongoDBStore};
use time::Duration;
use mongodb::Database;

#[derive(Clone,FromRef)]
pub struct AppState {
    pub db: DB,
    pub leptos_options: LeptosOptions,
    // pub config: Config,
}

pub struct App {
    database: Database,
}

impl App {
    pub async fn new() -> Self {
        Self {
            database: DB::connect().await.unwrap(),
        }
    }

    async fn generate_auth_layer(&self) -> ServiceBuilder<Stack<AuthManagerLayer<Backend, MongoDBStore>, Identity>> {
        let db_url = std::env::var("MONGODB_URI").expect("missing MONGODB_URI");
        let client = Client::with_uri_str(db_url).await.unwrap();
        let session_store = MongoDBStore::new(client, "tower-sessions".to_string());
        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::weeks(1)));

        let backend = Backend::new(&self.database).await;
        let auth_manager_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
        let auth_session_layer = ServiceBuilder::new().layer(auth_manager_layer);
            
        auth_session_layer
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let conf = get_configuration(None).unwrap();
        let leptos_options = conf.leptos_options;
        let appstate = AppState {
            db: DB::new(&self.database).await?,
            leptos_options: leptos_options.clone(),
        };
        
        let auth_layer = self.generate_auth_layer().await;
        
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(Frontend);

        let app = Router::new()
            .leptos_routes_with_context(
                &leptos_options,
                routes,
                {
                    let appstate_tmp = appstate.clone();
                    move || {
                        provide_context(appstate_tmp.clone());
                    }
                },
                {
                    let leptos_options = leptos_options.clone();
                    move || shell(leptos_options.clone())
                }
            )
            .layer(auth_layer)
            .fallback(file_and_error_handler::<LeptosOptions, _>(shell))
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        log!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

