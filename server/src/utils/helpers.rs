pub fn mongo_user_to_user(mongo_user: MongoUser) -> User {
    User {
        id: mongo_user.id.to_string(),
        name: mongo_user.name,
        email: mongo_user.email,
        hashed_password: mongo_user.hashed_password,
        account_type: mongo_user.account_type,
    }
}

pub fn user_to_mongo_user(user: User) -> MongoUser {
    MongoUser {
        id: ObjectId::new(),
        name: user.name,
        email: user.email,
        hashed_password: user.hashed_password,
        account_type: user.account_type,
    }
}
