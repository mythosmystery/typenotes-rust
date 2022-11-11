mod db;
mod models;
mod schema;

extern crate juniper;

use juniper::EmptySubscription;
use rocket::{response::content, State};
use schema::{Context, Mutation, Query, Schema};

use crate::db::mongo::MongoRepo;

#[rocket::get("/")]
fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &*context).await
}

#[rocket::main]
async fn main() {
    let ctx = init_context().await;
    let _ = rocket::build()
        .manage(ctx)
        .manage(Schema::new(
            Query,
            Mutation,
            EmptySubscription::<Context>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch()
        .await
        .expect("server to launch");
}

async fn init_context() -> Context {
    let mongo_repo = MongoRepo::new().await;
    let user_repo = db::user_repo::UserRepo::new(&mongo_repo);
    let note_repo = db::note_repo::NoteRepo::new(&mongo_repo);

    Context {
        mongo_repo,
        user_repo,
        note_repo,
    }
}
