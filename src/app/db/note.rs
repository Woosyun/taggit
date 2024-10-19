use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson,
    bson::oid::ObjectId,
    options::FindOptions,
};
// use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;
use crate::app::models::{Note, NoteItem};


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
    pub async fn insert_one(&self, note: Note) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(note).await
    }
    pub async fn find_one(&self, id: ObjectId) -> Result<Option<Note>, Error> {
        self.collection.find_one(bson::doc! { "id": id }).await
    }
    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<NoteItem>, Error> {
        let projection = bson::doc! {
            "id": 1,
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
            .await?
            .into_iter()
            .map(NoteItem::from)
            .collect::<Vec<NoteItem>>();

        Ok(note_items)
    }
}