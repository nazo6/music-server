use async_graphql::{EmptySubscription, Schema};

pub mod guard;
pub mod mutation;
pub mod output_objects;
pub mod query;
pub mod subscription;

pub type ApiSchema = Schema<query::QueryRoot, mutation::MutationRoot, EmptySubscription>;
