use juniper::futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use crate::models::note_model::{NewNote, Note};

use super::mongo::MongoRepo;

pub struct NoteRepo {
    collection: Collection<Note>,
}

impl NoteRepo {
    pub fn new(mongo_repo: &MongoRepo) -> Self {
        let collection = mongo_repo.db.collection::<Note>("Note");
        Self { collection }
    }

    pub async fn create(&self, data: NewNote) -> Result<Note, Box<dyn std::error::Error>> {
        let note = data.to_note();
        self.collection.insert_one(&note, None).await?;
        Ok(note)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Note, Box<dyn std::error::Error>> {
        let obj_id = ObjectId::parse_str(id)?;
        match self.collection.find_one(doc! {"_id": obj_id}, None).await? {
            Some(note) => Ok(note),
            None => Err("Note not found".into()),
        }
    }

    pub async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let obj_id = ObjectId::parse_str(user_id)?;
        let notes = self
            .collection
            .find(doc! {"user_id": obj_id}, None)
            .await?
            .try_collect::<Vec<Note>>()
            .await?;
        Ok(notes)
    }
}
