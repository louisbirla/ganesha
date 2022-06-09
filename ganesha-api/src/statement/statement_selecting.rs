use super::Statement;
use crate::graphql::Context;
use async_graphql::*;
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

#[derive(Default)]
pub struct StatementSelectingQueries;

#[Object]
impl StatementSelectingQueries {
    /// Tries to find a statement with the matching ID
    async fn find_one_statement(
        &self,
        context: &async_graphql::Context<'_>,
        id: String,
    ) -> Result<Option<Statement>> {
        let (db, _) = Context::parse(context)?;

        let result = Statement::collection(&db)
            .find_one(doc! { "_id": ObjectId::from_str(&id)? }, None)
            .await?;

        Ok(result)
    }
}
