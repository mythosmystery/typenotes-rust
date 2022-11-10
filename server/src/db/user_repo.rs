use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};

use crate::models::user::MongoUser;

use super::mongo::MongoRepo;

pub struct UserRepo {
    pub collection: Collection<MongoUser>,
}

impl UserRepo {
    pub fn new(mongo_repo: &MongoRepo) -> Self {
        let collection = mongo_repo.db.collection::<MongoUser>("User");
        Self { collection }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<MongoUser, Box<dyn std::error::Error>> {
        let obj_id = ObjectId::parse_str(id)?;
        match self.collection.find_one(doc! {"_id": obj_id}, None).await? {
            Some(user) => Ok(user),
            None => Err("User not found".into()),
        }
    }

    pub async fn find(&self, query: Document) -> Result<MongoUser, Box<dyn std::error::Error>> {
        match self.collection.find_one(query, None).await? {
            Some(user) => Ok(user),
            None => Err("User not found".into()),
        }
    }

    pub async fn create(&self, user: MongoUser) -> Result<MongoUser, Box<dyn std::error::Error>> {
        let result = self.collection.insert_one(&user, None).await?;
        Ok(MongoUser {
            id: result.inserted_id.as_object_id().unwrap_or_default(),
            name: user.name,
            email: user.email,
            hashed_password: user.hashed_password,
            account_type: user.account_type,
        })
    }
}
