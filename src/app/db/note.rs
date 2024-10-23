use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson,
    bson::DateTime,
    options::FindOptions,
};
// use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;
use crate::app::models::Note;
// use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct NoteService {
    collection: Collection::<Note>,
}

impl NoteService {
    pub fn new(collection: Collection::<Note>) -> Self {
        Self {
            collection
        }
    }
    // note id 자동 생성?
    pub async fn insert_one(&self, mut new_note: Note) -> Result<InsertOneResult, Error> {
        // let mut new_note = Note::new(
        //     title, 
        //     body, 
        //     tags, 
        //     author_id, 
        // ).map_err(|err| Error::custom(err))?;

        use leptos::logging::log;
        log!("(insert_one)inserting note {:?}", new_note);
        
        new_note.set_last_modified(DateTime::now().to_string());
        
        self.collection.insert_one(new_note).await
    }
    pub async fn find_one(&self, id: String) -> Result<Option<Note>, Error> {
        self.collection.find_one(bson::doc! { "id": id }).await
    }
    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<Note>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "title": 1,
            "author": 1,
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
            .try_collect::<Vec<Note>>()
            .await?;

        Ok(note_items)
    }
}