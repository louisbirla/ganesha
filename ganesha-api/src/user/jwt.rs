use crate::errors::{GaneshaError, UserSideError};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

/// Create a JSON web token with the user id encoded to keep track of logged in user
pub fn create_token(user_id: ObjectId) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::days(21)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_ref()),
    )
    .unwrap()
}

/// Decode a JWT to the user's ID
pub fn validate_token(token: &str) -> Result<ObjectId, GaneshaError> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_ref()),
        &Validation::default(),
    )?;

    Ok(token
        .claims
        .sub
        .parse()
        .expect("token content to be object ID"))
}

/// Sugar for extracting the string from Option<String> and returning auth error if it's
/// None
pub fn require_token(token: &Option<String>) -> Result<String, UserSideError> {
    match token {
        None => Err(UserSideError::AuthTokenRequired),
        Some(token) => Ok(token.to_string()),
    }
}

/// Combines validate_token and require_token for some sugar
pub fn require_valid_token(token: &Option<String>) -> Result<ObjectId, GaneshaError> {
    validate_token(&require_token(token)?)
}

/// Validates the token only if it is supplied
pub fn optional_valid_token(token: &Option<String>) -> Result<Option<ObjectId>, GaneshaError> {
    if let Some(token) = token {
        Ok(Some(validate_token(token)?))
    } else {
        Ok(None)
    }
}

/// Get the session secret key from the environment, needs to be set for auth
/// stuff to work
pub fn get_secret() -> String {
    dotenv::dotenv().ok();
    match std::env::var("SESSION_SECRET") {
        Ok(scrt) => scrt,
        Err(_) => {
            error!("A 'SESSION_SECRET' environment variable is required.");
            panic!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// The parameters that go into the JWT
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}
