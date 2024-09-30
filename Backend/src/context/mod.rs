mod error;

pub use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Context {
    user_id: i64,
}

// Constructor.
impl Context {
    pub fn root_ctx() -> Self {
        Context { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors.
impl Context {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
