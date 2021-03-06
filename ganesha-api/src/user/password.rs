use async_graphql::Error;
use rand::Rng;
use crate::errors::UserSideError;

/// Takes the rawtext password and hashes it
pub fn hash_pwd(password: String) -> Result<String, argon2::Error> {
	let password = password.as_bytes();
	let salt = rand::thread_rng().gen::<[u8; 32]>();

	// Apply Argon2i
	let argon_config = argon2::Config::default();
	argon2::hash_encoded(password, &salt, &argon_config)
}

/// Makes sure the password is long enough
pub fn validate_pwd(password: &str) -> Result<(), Error> {
	if password.len() < 8 {
		return Err(UserSideError::PasswordTooShort.into());
	};

	Ok(())
}

/// Makes sure that a password is both valid and correct with the hashed password.
/// Will error if the password is not valid, and will return false if it does not
/// match the hash.
pub fn verify_pwd(password: &str, hash: &str) -> Result<bool, Error> {
	validate_pwd(password)?;

	let password = password.as_bytes();

	Ok(argon2::verify_encoded(hash, password)?)
}
