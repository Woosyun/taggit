use super::Text;

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
pub struct TextService {
    collection: Collection::<Text>,
}

impl TextService {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<Text>("texts"),
        }
    }

    pub async fn insert_one(&self, mut text: Text) -> Result<InsertOneResult, Error> {
        text.set_id(bson::oid::ObjectId::new().to_hex());
        text.update_last_modified(DateTime::now().to_string());
        self.collection.insert_one(text).await
    }

    pub async fn find_one_by_id(&self, id: String) -> Result<Option<Text>, Error> {
        self.collection.find_one(bson::doc!{"_id": id}).await
    }

    pub async fn find_items_by_parent_id(&self, id: String) -> Result<Vec<Text>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "author_id": 1,
            "parent_id": 1,
            "title": 1,
            "tags": 1,
            "last_modified": 1,
        };
        
        let options = FindOptions::builder()
            .projection(projection)
            .build();

        let query = bson::doc! {"parent_id": id};

        let items = self.collection.find(query)
            .with_options(options)
            .await
            .map_err(|e| dbg!(e))?
            .try_collect()
            .await
            .map_err(|e| dbg!(e))?;

        Ok(items)
    }

    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<Text>, Error> {
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

        let items = self.collection.find(query)
            .with_options(options)
            .await
            .map_err(|e| dbg!(e))?
            .try_collect()
            .await
            .map_err(|e| dbg!(e))?;

        Ok(items)
    }
}