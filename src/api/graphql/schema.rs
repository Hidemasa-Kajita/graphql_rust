use super::{query_root::QueryRoot, subscription_root::SubscriptionRoot};
use async_graphql::{EmptyMutation, Schema};

pub type SampleSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub async fn generate_schema() -> Result<SampleSchema, sqlx::Error> {
    let pool = crate::database::connect::connect()
        .await
        .expect("Failed connect database.");
    Ok(Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
        .data(pool)
        .finish())
}
