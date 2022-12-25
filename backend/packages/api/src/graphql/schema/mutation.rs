use async_graphql::{Context, Object, Result};
use server_core::user::add_user;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn value(&self) -> String {
        "1".to_string()
    }
    async fn add_admin(
        &self,
        _ctx: &Context<'_>,
        #[graphql(desc = "Name of user")] name: String,
        #[graphql(desc = "Password of user")] password: String,
    ) -> Result<String> {
        add_user(&name, &password, true).await?;
        Ok("success".to_string())
    }
}
