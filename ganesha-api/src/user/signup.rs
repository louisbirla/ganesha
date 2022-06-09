use crate::graphql::Context;
use async_graphql::*;

use super::{
    auth_payload::AuthPayload,
    password::{hash_pwd, validate_pwd},
    username::{localize_username, validate_username},
    User,
};

#[derive(Default)]
pub struct SignupMutations;

#[Object]
impl SignupMutations {
    /// Verifies the username and password, then sends an email to confirm it.
    /// Returns a session code that must be
    /// sent along with the verification code using `confirmEmail`, which will then create the user.
    pub async fn signup(
        &self,
        context: &async_graphql::Context<'_>,
        username: String,
        password: String,
    ) -> Result<AuthPayload, Error> {
        let (db, _) = Context::parse(context)?;

        validate_pwd(&password)?;
        let hash = hash_pwd(password)?;

        let localuname = localize_username(&username);
        validate_username(&localuname, &db).await?;

        let new_user = User::new(username, hash, localuname);
        let user = User::collection(&db).insert_one(new_user, None).await?;

        Ok(AuthPayload::new(
            user.inserted_id
                .as_object_id()
                .expect("insert_id to be object id"),
        ))
    }
}
