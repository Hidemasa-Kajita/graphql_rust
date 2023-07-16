use async_graphql::{EmptyMutation, Schema};
use super::{query_root::QueryRoot, subscription_root::SubscriptionRoot};

pub type SampleSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub fn generate_schema() -> SampleSchema {
    Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot)
}
