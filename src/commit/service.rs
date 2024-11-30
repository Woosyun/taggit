#![allow(unused)]
use super::Commit;

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
pub struct CommitService {
    collection: Collection::<Commit>,
}

impl CommitService {
    pub fn new(db: &Database) -> Self {
        // let commit_col_name = std::env::var("MONGODB_COMMIT_COLLECTION_NAME")
        //     .expect("MONGODB_COMMIT_COLLECTION_NAME should be set");
        let collection = db.collection::<Commit>("commits");

        Self {
            collection
        }
    }

    pub async fn insert_one(&self, mut new_commit: Commit) -> Result<InsertOneResult, Error> {
        new_commit.set_id(bson::oid::ObjectId::new().to_hex());
        new_commit.update_last_modified(DateTime::now().to_string());
        self.collection.insert_one(new_commit).await
    }

    pub async fn find_one_by_id(&self, id: String) -> Result<Option<Commit>, Error> {
        self.collection.find_one(bson::doc!{"_id": id}).await
    }

    pub async fn find_items_by_parent_id(&self, id: String) -> Result<Vec<Commit>, Error> {
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

    pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<Commit>, Error> {
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
            bson::doc! {}
        } else {
            bson::doc! {"tags": { "$all": tags}}
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