use mongodb::Database;

/// The context to share among GraphQL requests
pub struct Context {
    /// Gives the GraphQL operations access to the DB
    pub db: Database,
    /// A JWT for authenticating a user with a request
    pub auth_token: Option<String>,
}

impl Context {
    pub fn parse(
        context: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<(Database, Option<String>)> {
        let context = context.data::<Context>()?;
        Ok((context.db.clone(), context.auth_token.clone()))
    }
}
