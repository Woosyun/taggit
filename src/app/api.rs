use leptos::{ServerFnError, logging::log, use_context};
use super::models::NoteItem;
#[cfg(feature = "ssr")]
use crate::app::db::DB;

#[leptos::server(Search, "/api")]
pub async fn search(tags: std::collections::HashSet<String>) -> Result<Vec<NoteItem>, ServerFnError> {
    // use crate::app::db::DB;

    let note_service = match use_context::<DB>() {
        Some(db) => db.note_service,
        None => return Err(ServerFnError::ServerError("cannot connect to database".to_string()))
    };

    log!("(search)searching notes by tags {:?}", tags);

    let note_items = note_service
        .find_items_by_tags(tags.into_iter().collect::<Vec<String>>())
        .await
        .map_err(|err| ServerFnError::ServerError(format!("error while fetching note items: {:?}", err)));

    {
        match &note_items {
            Ok(note_items) => {
                log!("(search)found note items: {:?}", note_items);
            }
            Err(err) => {
                log!("(search)error while fetching note items: {:?}", err);
            }
        }
    }
        
    note_items
}