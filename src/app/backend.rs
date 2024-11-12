use axum_macros::FromRef;
use leptos::prelude::LeptosOptions;
use crate::db;

#[derive(Clone,FromRef)]
pub struct AppState {
    pub db: db::DB,
    pub options: LeptosOptions,
    // pub config: Config,
}