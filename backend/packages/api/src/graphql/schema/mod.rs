use async_graphql::{
    Context, Data, EmptyMutation, EmptySubscription, Object, Result, Schema, Subscription,
};

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct Token(pub String);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_library(&self, ctx: &Context<'_>, name: String, path: String) -> String {
        "".to_string()
    }
}
