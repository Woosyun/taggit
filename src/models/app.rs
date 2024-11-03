use cfg_if::cfg_if;

cfg_if!{
    if #[cfg(feature="ssr")] {
        use axum_macros::FromRef;
        use leptos::prelude::*;
        use crate::db;
        
        #[derive(Clone,FromRef)]
        pub struct AppState {
            pub db: db::DB,
            pub options: LeptosOptions,
        }

        pub async fn server_fn_handler(){}
        pub async fn leptos_routes_handler(){}
    }
}