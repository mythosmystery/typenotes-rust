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

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub account_type: AccountType,
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
