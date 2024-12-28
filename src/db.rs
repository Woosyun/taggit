use mongodb::{error::Error, options::ClientOptions, Client, Database};
use crate::text::TextService;

#[derive(Clone, Debug)]
pub struct DB {
    pub text_service: TextService,
}

impl DB {
    pub async fn connect() -> Result<Database, Error> {
        let mongodb_uri = std::env::var("MONGODB_URI")
            .expect("MONGODB_URI must be set");
        let db_name = std::env::var("MONGODB_NAME")
            .expect("MONGODB_NAME must be set");
        
        //client_options을 만드는 중에 무슨 문제가?
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(db_name.clone()); 
        let client = Client::with_options(client_options)?;
        Ok(client.database(&db_name))
    }
    
    pub async fn new(database: &Database) -> Result<Self, Error> {
        // let database = Self::connect().await?;
        // let note_col_name = std::env::var("MONGODB_NOTE_COLLECTION_NAME")
            // .expect("MONGODB_NOTE_COLLECTION_NAME must be set");
        // let note_collection = database.collection::<Note>(&note_col_name);

        // println!("Connected to database: {}, note collection: {}", db_name, note_col_name);

        Ok(Self {
            text_service: TextService::new(database),
        })
    }
}