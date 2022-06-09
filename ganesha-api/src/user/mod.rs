use crate::{graphql::Context, statement::Statement};
use async_graphql::futures_util::TryStreamExt;
use async_graphql::*;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use self::jwt::optional_valid_token;

pub mod auth_payload;
pub mod jwt;
pub mod login;
mod password;
pub mod signup;
pub mod user_selecting;
mod username;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// Unique user identifier
    _id: ObjectId,
    /// Unique username, but not used for checking uniqueness because "louis" and "Louis" would be different
    username: String,
    /// A "localized" version of the username, used to check uniqueness of username
    localuname: String,
    /// The user's password hash (make sure not to expose)
    pwd_hash: String,
}

#[Object]
impl User {
    /// The user's unique ID. Use this whenever unique identification is needed
    /// (not username) because it cannot be changed.
    pub async fn id(&self) -> String {
        self._id.to_string()
    }

    /// The user's unique username. This can be changed, and no alphanumerically similar
    /// usernames may exist.
    async fn username(&self) -> String {
        self.username.clone()
    }

    /// Statements this user has made
    pub async fn statements(&self, context: &async_graphql::Context<'_>) -> Result<Vec<Statement>> {
        let (db, token) = Context::parse(context)?;
        let user_id = optional_valid_token(&token)?;

        let mut result = Statement::collection(&db)
            .find(doc! { "author": self._id }, None)
            .await?;

        let mut statements = vec![];

        // If you are the user, no need to check for public
        if Some(self._id) == user_id {
            while let Some(statement) = result.try_next().await? {
                statements.push(statement);
            }
        } else {
            while let Some(statement) = result.try_next().await? {
                // Only add public statements
                if statement.public == true {
                    statements.push(statement);
                }
            }
        }

        Ok(statements)
    }
}

impl User {
    fn new(username: String, pwd_hash: String, localuname: String) -> Self {
        Self {
            _id: ObjectId::new(),
            username,
            localuname,
            pwd_hash,
        }
    }
}

impl User {
    pub fn collection(db: &mongodb::Database) -> mongodb::Collection<Self> {
        db.collection("users")
    }
}
