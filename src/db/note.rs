use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use serde::{Serialize, Deserialize};
use futures::stream::TryStreamExt;

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    id: ObjectId,
    title: String,
    body: String,
    tags: Vec<String>,
    author: ObjectId,
    last_modified: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NoteItem {
    id: ObjectId,
    title: String,
    author: ObjectId,
    last_modified: u64,
}
impl From<Note> for NoteItem {
    fn from(note: Note) -> Self {
        NoteItem {
            id: note.id,
            title: note.title,
            author: note.author,
            last_modified: note.last_modified,
        }
    }
}

#[derive(Clone)]
pub struct NoteService {
    collection: Collection::<Note>,
}

impl NoteService {
    pub fn new(collection: Collection::<Note>) -> Self {
        Self {
            collection
        }
    }
    pub async fn insert_one(&self, note: Note) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(note).await
    }
    pub async fn find_one(&self, id: ObjectId) -> Result<Option<Note>, Error> {
        self.collection.find_one(doc! { "id": id }).await
    }
    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<NoteItem>, Error> {
        let projection = doc! {
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
            doc! {}
        } else {
            doc! {"tags": { "$all": tags}}
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