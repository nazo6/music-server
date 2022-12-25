use async_graphql::{Context, Guard, Result};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Role {
    Admin,
    User,
    Guest,
}

use std::cmp::Ordering;

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        use Role::*;
        match (self, other) {
            (Admin, Admin) | (User, User) | (Guest, Guest) => Some(Equal),
            (Admin, _) | (User, Guest) => Some(Greater),
            (Guest, _) | (User, Admin) => Some(Less),
        }
    }
}

pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if let Some(role) = ctx.data_opt::<Role>() {
            if role >= &self.role {
                Ok(())
            } else {
                Err("Unauthorized".into())
            }
        } else {
            Err("Internal error!".into())
        }
    }
}
