use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum, Serialize, Deserialize, Debug, Clone)]
pub enum AccountType {
    Admin,
    User,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub account_type: AccountType,
}

impl User {
    pub fn to_mongo_user(self) -> MongoUser {
        MongoUser {
            id: ObjectId::new(),
            name: self.name,
            email: self.email,
            hashed_password: self.hashed_password,
            account_type: self.account_type,
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
    pub fn to_mongo_user(self) -> MongoUser {
        MongoUser {
            id: ObjectId::new(),
            name: self.name,
            email: self.email,
            hashed_password: self.password,
            account_type: self.account_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoUser {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub account_type: AccountType,
}

impl MongoUser {
    pub fn to_user(self) -> User {
        User {
            id: self.id.to_string(),
            name: self.name,
            email: self.email,
            hashed_password: self.hashed_password,
            account_type: self.account_type,
        }
    }
}
