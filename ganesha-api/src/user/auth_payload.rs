use super::{jwt::create_token, User};
use crate::graphql::Context;
use async_graphql::*;
use mongodb::bson::{doc, oid::ObjectId};

/// An object that contains authentication information
pub struct AuthPayload {
    pub token: String,
    pub user_id: ObjectId,
}

#[Object]
impl AuthPayload {
    /// The user that is authenticated with the associated token (`token`)
    pub async fn user(&self, context: &async_graphql::Context<'_>) -> Result<Option<User>> {
        let (db, _) = Context::parse(context)?;

        let result = User::collection(&db)
            .find_one(doc! { "_id": self.user_id }, None)
            .await?;
        Ok(result)
    }
    /// The authentication JWT for the `user`. This is used to authenticate API requests
    /// based on the user when used in the authorization header.
    async fn token(&self) -> String {
        self.token.clone()
    }
}

impl AuthPayload {
    pub fn new(user_id: ObjectId) -> Self {
        AuthPayload {
            user_id,
            token: create_token(user_id),
        }
    }
}
