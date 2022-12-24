use async_graphql::{
    Context, Data, EmptyMutation, EmptySubscription, Object, Result, Schema, Subscription,
};

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct Token(pub String);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn value(&self) -> String {
        "1".to_string()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn value(&self) -> String {
        "1".to_string()
    }
}
