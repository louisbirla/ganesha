use crate::{
    graphql::Context,
    user::{
        jwt::{optional_valid_token, require_valid_token},
        User,
    },
};
use async_graphql::*;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use self::statement_position::StatementPosition;

pub mod statement_creating;
mod statement_position;
pub mod statement_selecting;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Statement {
    /// Unique statement identifier
    _id: ObjectId,
    /// The text content of the statement
    md_text: String,
    /// The user who created this statement, or just put it on Ganesha
    author: ObjectId,
    /// The statements that support this statement
    supported_by: Vec<ObjectId>,
    /// The statements that this statement supports
    supports: Vec<ObjectId>,
    /// Users and their positions on the statement
    user_positions: Vec<StatementPosition>,
    /// Is this statement public or private,
    pub public: bool,
    /// What this statement disproves
    disproves: Vec<ObjectId>,
    /// A name for referencing the statement
    title: String,
}

// TODO: Duplicates
// TODO: Height & Weight stats (how much relies on this, how much does this rely on)
// TODO: Compute whether or not you agree or disprove something that supports this

#[Object]
impl Statement {
    /// The statement's unique ID. Use this whenever unique identification is needed
    pub async fn id(&self) -> String {
        self._id.to_string()
    }

    /// The text content of the statement
    pub async fn md_text(&self) -> String {
        self.md_text.clone()
    }

    /// The text content of the statement
    pub async fn public(&self) -> bool {
        self.public
    }

    /// A recognizible name for the statement
    pub async fn title(&self) -> String {
        self.title.clone()
    }

    /// The author of this statement
    pub async fn author(&self, context: &async_graphql::Context<'_>) -> Result<Option<User>> {
        let (db, _) = Context::parse(context)?;

        let result = User::collection(&db)
            .find_one(doc! { "_id": self.author }, None)
            .await?;
        Ok(result)
    }

    /// The statements that support this
    pub async fn supported_by(
        &self,
        context: &async_graphql::Context<'_>,
    ) -> Result<Vec<Statement>> {
        let (db, token) = Context::parse(context)?;
        let user_id = optional_valid_token(&token)?;

        let mut supported_by = vec![];
        for id in self.supported_by.clone() {
            let statement = Statement::collection(&db)
                .find_one(doc! { "_id": id }, None)
                .await?;
            if let Some(statement) = statement {
                // Only public or your own statements can be shown
                if statement.public || Some(statement.author) == user_id {
                    supported_by.push(statement)
                }
            }
        }

        Ok(supported_by)
    }

    /// The statements that this supports
    pub async fn supports(&self, context: &async_graphql::Context<'_>) -> Result<Vec<Statement>> {
        let (db, token) = Context::parse(context)?;
        let user_id = optional_valid_token(&token)?;

        let mut supports = vec![];
        for id in self.supports.clone() {
            let statement = Statement::collection(&db)
                .find_one(doc! { "_id": id }, None)
                .await?;
            if let Some(statement) = statement {
                // Only public or your own statements can be shown
                if statement.public || Some(statement.author) == user_id {
                    supports.push(statement)
                }
            }
        }

        Ok(supports)
    }

    /// The statements that this disproves
    pub async fn disproves(&self, context: &async_graphql::Context<'_>) -> Result<Vec<Statement>> {
        let (db, token) = Context::parse(context)?;
        let user_id = optional_valid_token(&token)?;

        let mut statements = vec![];
        for id in self.disproves.clone() {
            let statement = Statement::collection(&db)
                .find_one(doc! { "_id": id }, None)
                .await?;
            if let Some(statement) = statement {
                // Only public or your own statements can be shown
                if statement.public || Some(statement.author) == user_id {
                    statements.push(statement)
                }
            }
        }

        Ok(statements)
    }

    pub async fn positions(
        &self,
        context: &async_graphql::Context<'_>,
    ) -> Result<Vec<StatementPosition>> {
        let (_, token) = Context::parse(context)?;
        let user_id = optional_valid_token(&token)?;

        let positions = self
            .user_positions
            .clone()
            .into_iter()
            .filter(|position| position.public || Some(position.user_id) == user_id)
            .collect();
        Ok(positions)
    }

    /// The position the authenticated user has on the statement
    pub async fn my_position(
        &self,
        context: &async_graphql::Context<'_>,
    ) -> Result<Option<StatementPosition>> {
        let (_, token) = Context::parse(context)?;
        let user_id = require_valid_token(&token)?;

        for position in &self.user_positions {
            if position.user_id == user_id {
                return Ok(Some(position.clone()));
            }
        }
        Ok(None)
    }
}

impl Statement {
    fn new(md_text: String, author: ObjectId, public: bool, title: String) -> Self {
        Self {
            _id: ObjectId::new(),
            md_text,
            author,
            supported_by: vec![],
            user_positions: vec![],
            public,
            supports: vec![],
            disproves: vec![],
            title,
        }
    }
}

impl Statement {
    pub fn collection(db: &mongodb::Database) -> mongodb::Collection<Self> {
        db.collection("statements")
    }
}
