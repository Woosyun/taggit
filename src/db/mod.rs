use mongodb::{error::Error, options::ClientOptions, Client};
use crate::note::{Note, NoteService};


#[derive(Clone, Debug)]
pub struct DB {
    pub note_service: NoteService,
}

impl DB {
    pub async fn new() -> Result<Self, Error> {
        let mongodb_uri = std::env::var("MONGODB_URI")
            .expect("MONGODB_URI must be set");
        let db_name = std::env::var("MONGODB_DATABASE_NAME")
            .expect("MONGODB_DB_NAME must be set");
        let note_col_name = std::env::var("MONGODB_NOTE_COLLECTION_NAME")
            .expect("MONGODB_NOTE_COLLECTION_NAME must be set");
        
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(db_name.clone());

        let client = Client::with_options(client_options)?;
        let database = client.database(&db_name);

        let note_collection = database.collection::<Note>(&note_col_name);

        println!("Connected to database: {}, note collection: {}", db_name, note_col_name);

        Ok(Self {
            note_service: NoteService::new(note_collection),
        })
    }
}