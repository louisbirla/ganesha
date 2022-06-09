use crate::{
    errors::UserSideError,
    graphql::Context,
    user::{password::verify_pwd, User},
};

use super::{auth_payload::AuthPayload, username::localize_username};
use async_graphql::*;
use mongodb::bson::doc;

#[derive(Default)]
pub struct LoginMutations;

#[Object]
impl LoginMutations {
    /// If the provided username and password are correct, will return with an
    /// authentication token & user pair for authenticating requests.
    pub async fn login(
        &self,
        context: &async_graphql::Context<'_>,
        username: String,
        password: String,
    ) -> Result<AuthPayload> {
        let (db, _) = Context::parse(context)?;

        let localuname = localize_username(&username);

        let result = User::collection(&db)
            .find_one(doc! { "localuname": localuname }, None)
            .await?;
        let user = match result {
            Some(user) => user,
            None => return Err(UserSideError::NoUserWithName(username).into()),
        };

        if !verify_pwd(&password, &user.pwd_hash)? {
            return Err(UserSideError::PasswordsDoNotMatch.into());
        }

        Ok(AuthPayload::new(user._id))
    }
}
