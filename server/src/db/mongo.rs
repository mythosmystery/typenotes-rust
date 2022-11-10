use mongodb::{Client, Database};

pub struct MongoRepo {
    pub db: Database,
}

impl MongoRepo {
    pub async fn new() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();

        let db = client.database("typenotes");
        Self { db }
    }
}
