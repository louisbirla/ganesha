use crate::{
    errors::{GaneshaError, UserSideError},
    user::User,
};
use mongodb::{bson::doc, Database};
use regex::Regex;

/// Turns a username into something more standard. Returns non-alphanumeric chars and
/// makes it lowercase
pub fn localize_username(username: &'_ str) -> String {
    let re = Regex::new(r"[^a-zA-Z\d]").unwrap();
    re.replace_all(username, "").to_string().to_lowercase()
}

/// Makes sure that a username is valid and not already used
pub async fn validate_username(localuname: &'_ str, db: &Database) -> Result<(), GaneshaError> {
    if localuname.len() < 3 {
        return Err(UserSideError::UsernameTooShort.into());
    }
    // A user with that name should not exist!
    let filter = doc! { "localuname": localuname };
    let result = User::collection(db).find_one(filter, None).await?;

    if result.is_some() {
        return Err(UserSideError::UsernameAlreadyUsed(localuname.to_string()).into());
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn localize_redundant() {
        assert_eq!("louis", localize_username("louis"));
    }

    #[test]
    fn localize_case() {
        assert_eq!("loop", localize_username("Loop"));
    }

    #[test]
    fn localize_numbers() {
        assert_eq!("number1", localize_username("Number1"));
    }

    #[test]
    fn localize_special() {
        assert_eq!("extracool", localize_username("EXTRA-COOL"));
    }
}
