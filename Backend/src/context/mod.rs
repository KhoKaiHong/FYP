mod error;

pub use self::error::{Error, Result};
use crate::auth::Role;

#[derive(Clone, Debug)]
pub struct Context {
    user_id: i64,
    role: Role,
}

// Constructor.
impl Context {
    pub fn root_ctx() -> Self {
        Context {
            user_id: 0,
            role: Role::User,
        }
    }

    pub fn new(user_id: i64, role: Role) -> Self {
        Context {
            user_id,
            role,
        }
    }
}

// Property Accessors.
impl Context {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn role(&self) -> &Role {
        &self.role
    }
}
