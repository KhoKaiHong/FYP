// region:    --- Modules

mod error;
pub mod password;
pub mod token;
pub mod role;

pub use self::error::{Error, Result};
pub use self::role::Role;

// endregion: --- Modules


// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
	use anyhow::Result;
    use crate::_dev_utils;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn password_test() -> Result<()> {
        // -- Exec
        let password = "test_password";
        let password_hashed = password::encrypt_password(password.to_owned()).await?;

        let validate_result = password::validate_password("test_password".to_owned(), password_hashed).await?;
        Ok(())
    }

    #[test]
    #[serial]
    fn access_token_test() -> Result<()> {
        
        Ok(())
    }


}

// endregion: --- Tests