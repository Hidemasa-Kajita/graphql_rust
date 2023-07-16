use async_graphql::{EmptyMutation, Schema};
use super::{query_root::QueryRoot, subscription_root::SubscriptionRoot};

pub type SampleSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub async fn generate_schema() -> Result<SampleSchema, sqlx::Error> {
    let pool = crate::database::connect::connect().await?;
    Ok(Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
        .data(pool)
        .finish()
    )
}
