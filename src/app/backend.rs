#![allow(unused)]

use axum_macros::FromRef;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes, file_and_error_handler};
use crate::{
    db,
    app::{Frontend, shell},
};
use axum::Router;
use axum_login::{
    AuthManagerLayerBuilder,
    tower_sessions::{
        ExpiredDeletion,
        Expiry,
        SessionManagerLayer,
    }
};
use tower_sessions_mongodb_store::{
    mongodb::Client,
    MongoDBStore,
};
use time::Duration;

#[derive(Clone,FromRef)]
pub struct AppState {
    pub db: db::DB,
    pub options: LeptosOptions,
    // pub config: Config,
}

pub struct App {
    db: db::DB,
    leptos_options: LeptosOptions,
}

impl App {
    pub async fn new() -> Self {
        let conf = get_configuration(None).unwrap();
        let options = conf.leptos_options;
        
        Self {
            db: db::DB::new().await.unwrap(),
            leptos_options: options,
        }
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let db_url = std::option_env!("MONGODB_URI").expect("missing MONGODB_URI");
        let client = Client::with_uri_str(db_url).await?;
        let session_store = MongoDBStore::new(client, "tower-sessions".to_string());
        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

        let backend = Backend::new();
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let appstate = AppState {
            options: self.leptos_options.clone(),
            db: self.db
        };
        let routes = generate_route_list(Frontend);
        let addr = self.leptos_options.site_addr;

        let app = Router::new()
            .leptos_routes_with_context(
                &self.leptos_options,
                routes,
                {
                    // let db = db.clone();
                    move || {
                        provide_context(appstate.clone());
                    }
                },
                {
                    let leptos_options = self.leptos_options.clone();
                    move || shell(leptos_options.clone())
                }
            )
            .layer(auth_layer)
            .fallback(file_and_error_handler::<LeptosOptions, _>(shell))
            .with_state(self.leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        log!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

