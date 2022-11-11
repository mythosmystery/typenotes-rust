use juniper::{integrations::chrono, GraphQLEnum, GraphQLInputObject, GraphQLObject};
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, Clone)]
pub enum AccountType {
    Admin,
    User,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(
        rename = "_id",
        with = "mongodb::bson::serde_helpers::hex_string_as_object_id"
    )]
    pub id: String,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub account_type: AccountType,
    #[serde(with = "mongodb::bson::serde_helpers::rfc3339_string_as_bson_datetime")]
    pub created_at: String,
    #[serde(with = "mongodb::bson::serde_helpers::rfc3339_string_as_bson_datetime")]
    pub updated_at: String,
}

impl User {
    pub fn new(email: String, name: String, hashed_password: String) -> User {
        User {
            id: ObjectId::new().to_string(),
            name,
            email,
            hashed_password,
            account_type: AccountType::User,
            created_at: DateTime::now().to_string(),
            updated_at: DateTime::now().to_string(),
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub account_type: AccountType,
}

impl NewUser {
    pub fn to_user(self) -> User {
        User {
            id: ObjectId::new().to_string(),
            name: self.name,
            email: self.email,
            hashed_password: self.password,
            account_type: self.account_type,
            created_at: DateTime::now().to_string(),
            updated_at: DateTime::now().to_string(),
        }
    }
}
