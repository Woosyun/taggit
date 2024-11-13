use super::User;

use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson,
    // bson::DateTime,
    // options::FindOptions,
};
// use serde::{Serialize, Deserialize};
// use futures::stream::TryStreamExt;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct UserService {
    collection: Collection::<User>,
}

impl UserService {
    pub fn new(collection: Collection::<User>) -> Self {
        Self {
            collection
        }
    }
    pub async fn insert_one(&self, mut new_user: User) -> Result<InsertOneResult, Error> {
        new_user.set_id(bson::oid::ObjectId::new().to_hex());
        self.collection.insert_one(new_user).await
    }
    pub async fn find_one(&self, id: String) -> Result<Option<User>, Error> {
        self.collection.find_one(bson::doc! { "_id": id }).await
    }
    // pub async fn find_items_by_tags(&self, tags: Vec<String>) -> Result<Vec<User>, Error> {
    //     let projection = bson::doc! {
    //         "_id": 1,
    //         "title": 1,
    //         "author_id": 1,
    //         "tags": 1,
    //         "last_modified": 1,
    //     };
        
    //     let options = FindOptions::builder()
    //         .limit(10)
    //         .projection(projection)
    //         .build();

    //     let query = if tags.is_empty() {
    //         bson::doc! {}
    //     } else {
    //         bson::doc! {"tags": { "$all": tags}}
    //     };
        
    //     let note_items = self.collection.find(query)
    //         .with_options(options)
    //         .await?
    //         .try_collect()
    //         .await?;

    //     // log!("(find_items_by_tags)found note items: {:?}", note_items);

    //     Ok(note_items)
    // }
}