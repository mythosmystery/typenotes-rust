use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use mongodb::bson::doc;

use crate::{
    db::{mongo::MongoRepo, user_repo::UserRepo},
    models::user::{NewUser, User},
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub struct Context {
    pub mongo_repo: MongoRepo,
    pub user_repo: UserRepo,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn get_user_by_id(context: &Context, id: String) -> FieldResult<User> {
        Ok(context.user_repo.find_by_id(&id).await?.to_user())
    }

    async fn get_user(context: &Context, email: String) -> FieldResult<User> {
        Ok(context
            .user_repo
            .find(doc! {"email": email})
            .await?
            .to_user())
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_user(context: &Context, mut data: NewUser) -> FieldResult<User> {
        data.password = bcrypt::hash(&data.password, 12)?;
        Ok(context
            .user_repo
            .create(data.to_mongo_user())
            .await?
            .to_user())
    }
}
