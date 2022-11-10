use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::models::user::{MongoUser, NewUser, User};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub struct Context {
    pub db: Database,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn user(context: &Context, id: String) -> FieldResult<User> {
        let obj_id = ObjectId::parse_str(&id)?;
        match context
            .db
            .collection::<MongoUser>("User")
            .find_one(doc! {"_id": obj_id}, None)
            .await?
        {
            Some(user) => Ok(User {
                id: user.id.to_hex(),
                name: user.name,
                email: user.email,
                hashed_password: user.hashed_password,
                account_type: user.account_type,
            }),
            None => Err("User not found".into()),
        }
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn createUser(context: &Context, mut data: NewUser) -> FieldResult<User> {
        let hashed_password = bcrypt::hash(&data.password, 12)?;

        let user = MongoUser {
            id: ObjectId::new(),
            name: data.name,
            email: data.email,
            hashed_password,
            account_type: data.account_type,
        };

        let result = context
            .db
            .collection::<MongoUser>("User")
            .insert_one(&user, None)
            .await?;

        Ok(User {
            id: result.inserted_id.to_string(),
            name: user.name,
            email: user.email,
            hashed_password: user.hashed_password,
            account_type: user.account_type,
        })
    }
}
