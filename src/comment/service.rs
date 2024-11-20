use super::Comment;

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
pub struct CommentService {
    collection: Collection::<Comment>,
}

impl CommentService {
    pub fn new(db: &Database) -> Self {
        let comment_col_name = std::env::var("MONGODB_COMMENT_COLLECTION_NAME")
            .expect("MONGODB_COMMENT_COLLECTION_NAME should be set");
        let collection = db.collection::<Comment>(&comment_col_name);

        Self {
            collection
        }
    }
    
    pub async fn insert_one(&self, mut new_comment: Comment) -> Result<InsertOneResult, Error> {
        new_comment.set_id(bson::oid::ObjectId::new().to_hex());
        new_comment.set_last_modified(DateTime::new().to_string());

        self.collection.insert_one(new_comment).await
    }
    pub async fn find_by_target_id(&self, target_id: String) -> Result<Vec<Comment>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "body": 1,
            "author_id", 1,
            "last_modified": 1,
            "target_position": 1,
        };
        let options = FindOptions::builder()
            .limit(5)
            .projection(projection)
            .build();

        let query = bson::doc! {"target_id": target_id};

        let comments = self.collection.find(query)
            .with_options(options)
            .await?
            .try_collect()
            .await?;

        Ok(comments)
    }
    
    pub async fn find_by_position(&self, target_id: String, target_position: String) -> Result<Vec<Comment>, Error> {
        let projection = bson::doc! {
            "_id": 1,
            "body": 1,
            "author_id", 1,
            "last_modified": 1,
            "target_position": 1,
        };
        let options = FindOptions::builder()
            .limit(5)
            .projection(projection)
            .build();

        let query = bson::doc! {"and": {"target_id": target_id, "target_position": target_position}};

        let comments = self.collection.find(query)
            .with_options(options)
            .await?
            .try_collect()
            .await?;

        Ok(comments)
    }
}