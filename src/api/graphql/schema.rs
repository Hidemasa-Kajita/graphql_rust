use super::dataloader::users::UserLoader;
use super::{subscription_root::SubscriptionRoot, QueryRoot};
use async_graphql::{dataloader::DataLoader, EmptyMutation, Schema};

pub type SampleSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub async fn generate_schema() -> Result<SampleSchema, sqlx::Error> {
    let pool = crate::database::connect::connect()
        .await
        .expect("Failed connect database.");
    Ok(
        Schema::build(QueryRoot::default(), EmptyMutation, SubscriptionRoot)
            .data(pool.clone())
            .data(DataLoader::new(
                UserLoader { pool: pool.clone() },
                tokio::spawn,
            ))
            .finish(),
    )
}
