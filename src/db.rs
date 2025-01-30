use mongodb::{error::Error, options::ClientOptions, Client, Database};
use crate::mail::MailService;

#[derive(Clone, Debug)]
pub struct DB {
    pub mail_service: MailService
}

impl DB {
    pub async fn connect() -> Result<Database, Error> {
        let mongodb_uri = std::env::var("MONGODB_URI")
            .expect("MONGODB_URI must be set");
        let db_name = std::env::var("MONGODB_NAME")
            .expect("MONGODB_NAME must be set");
        
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(db_name.clone()); 
        let client = Client::with_options(client_options)?;
        Ok(client.database(&db_name))
    }
    
    pub async fn new(database: &Database) -> Result<Self, Error> {
        Ok(Self {
            mail_service: MailService::new(database)
        })
    }
}
