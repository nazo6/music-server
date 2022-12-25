use super::guard::{Role, RoleGuard};
use async_graphql::{Context, Object, Result};
use server_core::user::create_user;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn test_guest(&self) -> String {
        "ok".to_string()
    }
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn test_user(&self) -> String {
        "ok".to_string()
    }
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn test_admin(&self) -> String {
        "ok".to_string()
    }
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn create_user(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Name of user")] name: String,
        #[graphql(desc = "Password of user")] password: String,
        #[graphql(desc = "Is admin user")] is_admin: bool,
    ) -> Result<String> {
        create_user(&name, &password, is_admin).await?;
        Ok("success".to_string())
    }
}
