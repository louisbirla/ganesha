use super::Statement;
use crate::{graphql::Context, user::jwt::require_valid_token};
use async_graphql::*;

#[derive(Default)]
pub struct StatementCreationMutations;

#[Object]
impl StatementCreationMutations {
    /// Creates a statement using the text, with the author as the logged in user.
    /// The supporting axioms should be listed in the `supported_by`, or this will
    /// be flagged as invalid.
    pub async fn create_statement(
        &self,
        context: &async_graphql::Context<'_>,
        md_text: String,
        title: String,
        #[graphql(default)] supported_by: Vec<String>,
        #[graphql(default)] public: bool,
    ) -> Result<Statement, Error> {
        let (db, token) = Context::parse(context)?;
        let user_id = require_valid_token(&token)?;

        let mut ids = vec![];
        for support_id in supported_by {
            ids.push(support_id.parse()?);
        }

        let new_statement = Statement {
            supported_by: ids,
            ..Statement::new(md_text, user_id, public, title)
        };
        let statement = Statement::collection(&db)
            .insert_one(new_statement.clone(), None)
            .await?;

        Ok(Statement {
            _id: statement
                .inserted_id
                .as_object_id()
                .expect("_id to be ObjectId"),
            ..new_statement
        })
    }
}
