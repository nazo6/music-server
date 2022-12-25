pub struct QueryRoot;

use async_graphql::{Context, Object, Result};
use server_core::user::get_users;

use super::{
    guard::{Role, RoleGuard},
    output_objects::User as UserResponse,
};
use crate::auth_extractor::User;

#[Object]
impl QueryRoot {
    /// Return the current authed user
    /// return null if not authed
    /// guard: user
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn me(&self, ctx: &Context<'_>) -> Result<UserResponse> {
        let user = ctx.data::<Option<User>>();
        if let Ok(Some(user)) = user {
            Ok(UserResponse {
                name: user.name.clone(),
                is_admin: user.is_admin,
            })
        } else {
            Err("Internal error!".into())
        }
    }
    /// Return the list of user
    /// guard: admin
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<UserResponse>> {
        let users = get_users().await?;

        users
            .iter()
            .map(|user| {
                Ok(UserResponse {
                    name: user.name.clone(),
                    is_admin: user.is_admin,
                })
            })
            .collect()
    }
}
