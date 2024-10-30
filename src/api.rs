use leptos::{server, ServerFnError};
use super::models::Note;
#[cfg(feature = "ssr")]
use crate::db::DB;

#[server(Search, "/api")]
pub async fn search(tags: Vec<String>) -> Result<Vec<Note>, ServerFnError> {
    use leptos::{logging::log, use_context};

    let note_service = match use_context::<DB>() {
        Some(db) => db.note_service,
        None => return Err(ServerFnError::ServerError("cannot connect to database".to_string()))
    };

    log!("(search)searching notes by tags {:?}", tags);

    let note_items = note_service
        .find_items_by_tags(tags)
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

#[server(InsertNote, "/api")]
pub async fn insert_note(
    title: String, 
    body: String, 
    tags: Vec<String>, 
    author_id: String
) -> Result<String, ServerFnError<String>> {
    use leptos::{use_context, logging::log};

    log!("(api/insert_note) entered");

    // let note_service = match use_context::<DB>() {
    //     Some(db) => db.note_service,
    //     None => return Err(ServerFnError::ServerError("cannot connect to database".to_string()))
    // };
    let note_service = use_context::<DB>()
        .map(|db| db.note_service)
        .ok_or(ServerFnError::ServerError("cannot connect to database".to_string()))?;

    log!("(api/insert_note) db connected.");

    let new_note = Note::new(
        title, 
        body, 
        tags, 
        author_id, 
    ).map_err(|err| ServerFnError::ServerError(err))?;

    log!("(api/insert_note) get new note: {:?}", new_note);

    note_service
        .insert_one(new_note)
        .await
        .map_err(|err| ServerFnError::ServerError(format!("error while inserting note: {:?}", err)))
        .map(|insert_one_result| insert_one_result.inserted_id.to_string())
}

#[server(FetchNoteById, "/api")]
pub async fn fetch_note_by_id(id: String) -> Result<Note, ServerFnError> {
    //check if the note exists
    use leptos::use_context;
    let note_service = match use_context::<DB>() {
        Some(db) => db.note_service,
        None => return Err(ServerFnError::ServerError("cannot connect to database".to_string()))
    };

    match note_service.find_one(id).await {
        Ok(note_option) => {
            match note_option {
                Some(note) => Ok(note),
                None => Err(ServerFnError::ServerError("note not found".to_string()))
            }
        },
        Err(err) => Err(ServerFnError::ServerError(format!("error while fetching note: {:?}", err)))
    }
}