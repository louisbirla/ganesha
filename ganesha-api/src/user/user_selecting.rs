use super::{
    jwt::{require_token, validate_token},
    username::localize_username,
    User,
};
use crate::graphql::Context;
use async_graphql::*;
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

#[derive(Default)]
pub struct UserSelectingQueries;

#[Object]
impl UserSelectingQueries {
    /// Tries to find a user with a matching ID or username
    async fn find_one_user(
        &self,
        context: &async_graphql::Context<'_>,
        id: Option<String>,
        username: Option<String>,
    ) -> Result<Option<User>> {
        let (db, _) = Context::parse(context)?;

        let mut filter = doc! {};
        if let Some(id) = id {
            filter.insert("_id", ObjectId::from_str(&id)?);
        }
        if let Some(username) = username {
            filter.insert("localuname", localize_username(&username));
        }

        let result = User::collection(&db).find_one(filter, None).await?;

        Ok(result)
    }

    /// The user that reflects the authorization token
    async fn whoami(&self, context: &async_graphql::Context<'_>) -> Result<Option<User>> {
        let (db, token) = Context::parse(context)?;
        let user_id = validate_token(&require_token(&token)?)?;

        let result = User::collection(&db)
            .find_one(doc! { "_id": user_id }, None)
            .await?;
        Ok(result)
    }
}
