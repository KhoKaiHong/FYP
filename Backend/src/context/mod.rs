// Modules
use crate::auth::Role;
use uuid::Uuid;

// Context for the application.
#[derive(Clone, Debug)]
pub struct Context {
    user_id: i64,
    role: Role,
    token_id: Uuid,
}

impl Context {
    // Generate a context for root user
    pub fn root_context() -> Self {
        Context {
            user_id: 0,
            role: Role::User,
            token_id: Uuid::new_v4(),
        }
    }

    // Create a context for a user
    pub fn new(user_id: i64, role: Role, token_id: Uuid) -> Self {
        Context {
            user_id,
            role,
            token_id,
        }
    }

    // Get the user id
    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    // Get the role
    pub fn role(&self) -> &Role {
        &self.role
    }

    // Get the token id
    pub fn token_id(&self) -> Uuid {
        self.token_id
    }
}
