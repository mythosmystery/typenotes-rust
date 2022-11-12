use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, Clone)]
pub enum NoteType {
    Text,
    Code,
    Html,
    Image,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    #[serde(
        rename = "_id",
        with = "mongodb::bson::serde_helpers::hex_string_as_object_id"
    )]
    pub id: String,
    pub title: String,
    pub content: String,
    pub note_type: NoteType,
    #[serde(with = "mongodb::bson::serde_helpers::hex_string_as_object_id")]
    pub user_id: String,
    #[serde(with = "mongodb::bson::serde_helpers::rfc3339_string_as_bson_datetime")]
    created_at: String,
    #[serde(with = "mongodb::bson::serde_helpers::rfc3339_string_as_bson_datetime")]
    updated_at: String,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct NewNote {
    pub title: String,
    pub content: String,
    pub note_type: NoteType,
    pub user_id: String,
}

impl NewNote {
    pub fn to_note(self) -> Note {
        Note {
            id: ObjectId::new().to_string(),
            title: self.title,
            content: self.content,
            note_type: self.note_type,
            user_id: self.user_id,
            created_at: DateTime::now().try_to_rfc3339_string().unwrap(),
            updated_at: DateTime::now().try_to_rfc3339_string().unwrap(),
        }
    }
}
