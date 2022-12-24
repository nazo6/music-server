use async_graphql::{
    Context, Data, EmptyMutation, EmptySubscription, Object, Result, Schema, Subscription,
};
use server_core::user::add_user;

use crate::auth_extractor::{ExtractUser, User};

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn value(&self) -> String {
        "1".to_string()
    }
    async fn user(&self, ctx: &Context<'_>) -> Option<String> {
        let user = ctx.data::<User>();
        if let Ok(user) = user {
            user.0.as_ref().map(|user| user.name.clone())
        } else {
            None
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn value(&self) -> String {
        "1".to_string()
    }
    async fn add_admin(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Name of user")] name: String,
        #[graphql(desc = "Password of user")] password: String,
    ) -> Result<String> {
        add_user(&name, &password, true).await?;
        Ok("success".to_string())
    }
}
