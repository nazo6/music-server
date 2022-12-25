pub struct QueryRoot;

use async_graphql::{Context, Object, SimpleObject};

use crate::auth_extractor::User;

#[derive(SimpleObject)]
struct UserResponse {
    name: String,
    is_admin: bool,
}

#[Object]
impl QueryRoot {
    /// Return the current authed user
    async fn user(&self, ctx: &Context<'_>) -> Option<UserResponse> {
        let user = ctx.data::<Option<User>>();
        if let Ok(Some(user)) = user {
            Some(UserResponse {
                name: user.name.clone(),
                is_admin: user.is_admin,
            })
        } else {
            None
        }
    }
}
