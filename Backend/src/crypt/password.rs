use super::{Error, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

// tokio::task::spawn_blocking is used as argon2 will block the async executor. 
// With many requests, OS thread will be held and cannot be moved.

// Encrypt the password using argon2 algorithm
pub async fn encrypt_password(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| Error::FailedToHashPassword)?
            .to_string();

        Ok(password_hash)
    })
    .await
    .map_err(|_| Error::FailSpawnBlockForEncrypt)?
}

// Validate a password using the same algorithm
pub async fn validate_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let parsed_hash =
            PasswordHash::new(&password_hash).map_err(|_| Error::PasswordHashWrongFormat)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::PasswordNotMatching)
    })
    .await
    .map_err(|_| Error::FailSpawnBlockForValidate)?
}
