use super::{Error, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

// tokio::task::spawn_blocking is used as argon2 will block the async executor. 
// With many requests, OS thread will be held and cannot be moved.

// Encrypt the password using argon2 algorithm
pub async fn encrypt_password(password: &str) -> Result<String> {
    let password = password.to_owned();

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
pub async fn validate_password(password: &str, password_hash: &str) -> Result<()> {
    let password = password.to_owned();
    let password_hash = password_hash.to_owned();

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

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
	use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn password_test() -> Result<()> {
        // -- Exec
        let password = "test_password";
        let password_hashed = encrypt_password(password).await?;

        let validate_result = validate_password("test_password", &password_hashed).await?;
        Ok(())
    }
}

// endregion: --- Tests
