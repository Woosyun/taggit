use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson,
    options::FindOptions,
    Database,
};
use futures::TryStreamExt;
use crate::note::{Note, NoteItem};

pub struct NoteService {
    collection: Collection::<Note>
}
impl NoteService {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<Note>("texts"),
        }
    }

    pub async fn insert_one(&self, mut note: Note) -> Result<InsertOneResult, Error> {
        note.set_id(bson::oid::ObjectId::new().to_hex());
        note.update_last_modified(bson::DateTime::now().to_string());
        self.collection.insert_one(note).await
    }

    pub async fn find_one_by_id(&self, id: String) -> Result<Option<Note>, Error> {
        self.collection.find_one(bson::doc!{"_id": id}).await
    }
    pub async fn find_items_by_parent_id(&self, id: String) -> Result<Vec<NoteItem>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "author_id": 1,
            "parent_id": 1,
            "title": 1,
            "tags": 1,
            "last_modified": 1
        };
        let options = FindOptions::builder()
            .projection(projection)
            .build();
        let query = bson::doc! {"parent_id": id};

        self.collection.find(query)
            .with_options(options)
            .await
            .map_err(|e| dbg!(e))?
            .try_collect()
            .await
            .map_err(|e| dbg!(e))
            .map(|notes: Vec<Note>| {
                notes.into_iter().map(Into::into).collect()
            })
    }
    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<NoteItem>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "author_id": 1,
            "parent_id": 1,
            "title": 1,
            "tags": 1,
            "last_modified": 1,
        };
        let options = FindOptions::builder()
            .limit(10)
            .projection(projection)
            .build();
        let query = if tags.is_empty() {
            bson::doc! {"parent_id": {"$eq": bson::Bson::Null}}
        } else {
            bson::doc! {"tags": { "$all": tags}, "parent_id": {"$eq": bson::Bson::Null}}
        };

        self.collection.find(query)
            .with_options(options)
            .await
            .map_err(|e| dbg!(e))?
            .try_collect()
            .await
            .map_err(|e| dbg!(e))
            .map(|notes: Vec<Note>| {
                notes.into_iter().map(Into::into).collect()
            })
    }
}