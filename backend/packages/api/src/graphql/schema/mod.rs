use async_graphql::{EmptySubscription, Schema};

pub mod guard;
pub mod mutation;
pub mod query;

pub type ApiSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;
