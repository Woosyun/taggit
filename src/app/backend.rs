use axum_macros::FromRef;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes, file_and_error_handler};
use crate::{
    db::DB,
    app::{Frontend, shell},
    auth::Backend,
};
use axum::Router;
use axum_login::AuthManagerLayerBuilder;
// use tower::{layer::util::{Identity, Stack}, ServiceBuilder};
use tower_sessions::{
    session_store::ExpiredDeletion,
    Expiry,
    SessionManagerLayer,
    // SessionStore,
};
use tokio::{signal, task::AbortHandle};
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

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let conf = get_configuration(None).unwrap();
        let leptos_options = conf.leptos_options;
        let appstate = AppState {
            db: DB::new(&self.database).await?,
            leptos_options: leptos_options.clone(),
        };
        
        // let auth_layer = self.generate_auth_layer().await;
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
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();
        
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
        // log!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(Self::shutdown_signal(deletion_task.abort_handle()))
            .await?;

        Ok(())
    }

    async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };
    
        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };
    
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
    
        tokio::select! {
            _ = ctrl_c => { deletion_task_abort_handle.abort() },
            _ = terminate => { deletion_task_abort_handle.abort() },
        }
    }
}

