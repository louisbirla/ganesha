use crate::{
    statement::{
        statement_creating::StatementCreationMutations,
        statement_selecting::StatementSelectingQueries,
    },
    user::{login::LoginMutations, signup::SignupMutations, user_selecting::UserSelectingQueries},
};

use super::misc_queries::MiscQueries;
use async_graphql::{EmptySubscription, MergedObject, Schema as GraphQLSchema};

#[derive(MergedObject, Default)]
pub struct Query(MiscQueries, UserSelectingQueries, StatementSelectingQueries);

#[derive(MergedObject, Default)]
pub struct Mutation(LoginMutations, SignupMutations, StatementCreationMutations);

pub type Schema = GraphQLSchema<Query, Mutation, EmptySubscription>;

/// Combines all the GraphQL resolvers into a schema
pub fn build_schema() -> Schema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish()
}
