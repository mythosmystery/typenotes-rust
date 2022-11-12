use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use super::user_model::User;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct AuthResult {
    pub token: String,
    pub user: User,
}
