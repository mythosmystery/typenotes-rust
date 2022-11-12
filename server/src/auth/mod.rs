use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub user_id: String,
    pub exp: i64,
}

impl Token {
    pub fn new(user_id: String) -> Token {
        Token {
            user_id,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp(),
        }
    }
    pub fn to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(TOKEN_SECRET),
        )?;
        Ok(token)
    }
    pub fn parse(token: String) -> Result<Self, jsonwebtoken::errors::Error> {
        let token = jsonwebtoken::decode::<Self>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(TOKEN_SECRET),
            &jsonwebtoken::Validation::default(),
        )?;
        Ok(token.claims)
    }
}

pub const TOKEN_EXPIRATION: i64 = 60 * 60 * 24 * 7; // 7 days
pub const TOKEN_SECRET: &[u8] = "suuuuuuper secret secrete!!!!!!".as_bytes();
pub const NO_AUTH_OPS: &[&str] = &["login", "register"];
