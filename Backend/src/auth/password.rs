// Modules
use crate::auth::{Error, Result};
use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;

// Encrypt the password using argon2 algorithm
pub async fn encrypt_password(password: &str) -> Result<String> {
    let password = password.to_owned();

    // As argon2 is blocking, we need to spawn a blocking task to prevent async executer to be blocked
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

// Validate a password using the argon2 algorithm
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn password_test() -> Result<()> {
        let password = "test_password";
        let password_hashed = encrypt_password(password).await?;

        validate_password("test_password", &password_hashed).await?;
        Ok(())
    }
}
