pub struct QueryRoot;

use async_graphql::{Context, Object};

use crate::auth_extractor::User;

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
