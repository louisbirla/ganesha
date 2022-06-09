use crate::{graphql::Context, user::User};
use async_graphql::*;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Enum)]
pub enum PositionType {
    Agree,
    Disagree,
    NotSure,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatementPosition {
    /// User whose position this is
    pub user_id: ObjectId,
    /// Is this position public or private
    pub public: bool,
    /// The user's stance on the statement
    position_type: PositionType,
}

#[Object]
impl StatementPosition {
    /// The holder of this position
    pub async fn user(&self, context: &async_graphql::Context<'_>) -> Result<Option<User>> {
        let (db, _) = Context::parse(context)?;

        let result = User::collection(&db)
            .find_one(doc! { "_id": self.user_id }, None)
            .await?;
        Ok(result)
    }

    /// What stance does the user have on the statement?
    pub async fn position_type(&self) -> PositionType {
        self.position_type.clone()
    }

    /// Is this statement private to the user or public to the world?
    pub async fn public(&self) -> bool {
        self.public
    }
}

impl StatementPosition {
    fn new(user_id: ObjectId, position_type: PositionType, public: bool) -> Self {
        Self {
            user_id,
            public,
            position_type,
        }
    }
}
