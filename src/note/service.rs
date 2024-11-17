use super::Note;

use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson,
    bson::DateTime,
    options::FindOptions,
    Database,
};
// use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct NoteService {
    collection: Collection::<Note>,
}

impl NoteService {
    pub fn new(db: &Database) -> Self {
        let note_col_name = std::env::var("MONGODB_NOTE_COLLECTION_NAME")
            .expect("MONGODB_NOTE_COLLECTION_NAME must be set");
        let collection = db.collection::<Note>(&note_col_name);

        Self {
            collection
        }
    }
    // pub fn new(collection: Collection::<Note>) -> Self {
    //     Self {
    //         collection
    //     }
    // }
    pub async fn insert_one(&self, mut new_note: Note) -> Result<InsertOneResult, Error> {
        new_note.set_id(bson::oid::ObjectId::new().to_hex());
        new_note.set_last_modified(DateTime::now().to_string());
        self.collection.insert_one(new_note).await
    }
    pub async fn find_one_by_id(&self, id: String) -> Result<Option<Note>, Error> {
        self.collection.find_one(bson::doc! { "_id": id }).await
    }
    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<Note>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "title": 1,
            "author_id": 1,
            "tags": 1,
            "last_modified": 1,
        };
        
        let options = FindOptions::builder()
            .limit(10)
            .projection(projection)
            .build();

        let query = if tags.is_empty() {
            bson::doc! {}
        } else {
            bson::doc! {"tags": { "$all": tags}}
        };
        
        let note_items = self.collection.find(query)
            .with_options(options)
            .await?
            .try_collect()
            .await?;

        // log!("(find_items_by_tags)found note items: {:?}", note_items);

        Ok(note_items)
    }
}