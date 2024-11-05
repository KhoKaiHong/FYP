mod error;

use uuid::Uuid;

pub use self::error::{Error, Result};
use crate::auth::Role;

#[derive(Clone, Debug)]
pub struct Context {
    user_id: i64,
    role: Role,
    token_id: Uuid,
}

// Constructor.
impl Context {
    pub fn root_ctx() -> Self {
        Context {
            user_id: 0,
            role: Role::User,
            token_id: Uuid::new_v4(),
        }
    }

    pub fn new(user_id: i64, role: Role, token_id: Uuid) -> Self {
        Context {
            user_id,
            role,
            token_id,
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

    pub fn token_id(&self) -> Uuid {
        self.token_id
    }
}
