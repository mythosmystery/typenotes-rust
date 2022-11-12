use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use mongodb::bson::doc;

use crate::{
    db::{mongo::MongoRepo, note_repo::NoteRepo, user_repo::UserRepo},
    models::{
        auth_model::AuthResult,
        note_model::{NewNote, Note},
        user_model::{NewUser, User},
    },
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub struct Context {
    pub mongo_repo: MongoRepo,
    pub user_repo: UserRepo,
    pub note_repo: NoteRepo,
    pub user_id: Option<String>,
}

impl juniper::Context for Context {}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn get_user_by_id(context: &Context, id: String) -> FieldResult<User> {
        Ok(context.user_repo.find_by_id(&id).await?)
    }

    async fn get_user(context: &Context, email: String) -> FieldResult<User> {
        Ok(context.user_repo.find(doc! {"email": email}).await?)
    }

    async fn get_note_by_id(context: &Context, id: String) -> FieldResult<Note> {
        Ok(context.note_repo.find_by_id(&id).await?)
    }

    async fn get_notes_by_user(context: &Context, user_id: String) -> FieldResult<Vec<Note>> {
        Ok(context.note_repo.find_by_user(&user_id).await?)
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_user(context: &Context, mut data: NewUser) -> FieldResult<User> {
        data.password = bcrypt::hash(&data.password, bcrypt::DEFAULT_COST)?;
        Ok(context.user_repo.create(data).await?)
    }

    async fn create_note(context: &Context, data: NewNote) -> FieldResult<Note> {
        Ok(context.note_repo.create(data).await?)
    }

    async fn register(
        context: &Context,
        email: String,
        name: String,
        password: String,
    ) -> FieldResult<AuthResult> {
        Ok(context.user_repo.register(email, name, password).await?)
    }

    async fn login(context: &Context, email: String, password: String) -> FieldResult<AuthResult> {
        Ok(context.user_repo.login(email, password).await?)
    }
}
