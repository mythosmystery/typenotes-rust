use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
};

use crate::models::user_model::{NewUser, User};

use super::mongo::MongoRepo;

pub struct UserRepo {
    collection: Collection<User>,
}

impl UserRepo {
    pub fn new(mongo_repo: &MongoRepo) -> Self {
        let collection = mongo_repo.db.collection::<User>("User");
        Self { collection }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<User, Box<dyn std::error::Error>> {
        let obj_id = ObjectId::parse_str(id)?;
        match self.collection.find_one(doc! {"_id": obj_id}, None).await? {
            Some(user) => Ok(user),
            None => Err("User not found".into()),
        }
    }

    pub async fn find(&self, query: Document) -> Result<User, Box<dyn std::error::Error>> {
        match self.collection.find_one(query, None).await? {
            Some(user) => Ok(user),
            None => Err("User not found".into()),
        }
    }

    pub async fn create(&self, data: NewUser) -> Result<User, Box<dyn std::error::Error>> {
        let user = data.to_user();
        self.collection.insert_one(&user, None).await?;
        Ok(user)
    }

    pub async fn register(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        let user = User::new(email, name, hashed_password);
        self.collection.insert_one(&user, None).await?;
        Ok(user)
    }
}
