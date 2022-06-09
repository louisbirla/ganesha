use thiserror::Error;

#[derive(Debug, Error)]
pub enum GaneshaError {
    #[error("{0}")]
    UserSideError(UserSideError),
    #[error("An unknown error occurred.")]
    UnknownError,
}

#[derive(Debug, Error)]
pub enum UserSideError {
    #[error("A user's password must be 8+ chars.")]
    PasswordTooShort,
    #[error("A username must be 3+ chars.")]
    UsernameTooShort,
    #[error("The username '{0}' is already in use.")]
    UsernameAlreadyUsed(String),
    #[error("There is no user with the username '{0}'.")]
    NoUserWithName(String),
    #[error("The user's password and the provided password do not match.")]
    PasswordsDoNotMatch,
    #[error("An authentication token is required.")]
    AuthTokenRequired,
    #[error("An object ID provided was invalid (was provided as a string and couldn't compile).")]
    InvalidObjectId,
    #[error("You have already left a review for this object.")]
    ReviewAlreadyExisted,
}

impl From<UserSideError> for GaneshaError {
    fn from(e: UserSideError) -> Self {
        GaneshaError::UserSideError(e)
    }
}

impl From<mongodb::error::Error> for GaneshaError {
    fn from(_: mongodb::error::Error) -> Self {
        GaneshaError::UnknownError
    }
}

impl From<jsonwebtoken::errors::Error> for GaneshaError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        GaneshaError::UnknownError
    }
}
