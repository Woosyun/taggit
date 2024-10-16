use mongodb::bson::oid::ObjectId;

struct Note {
    id: ObjectId,
    title: String,
    body: String,
    tags: Vec<String>,
    author: ObjectId,
}