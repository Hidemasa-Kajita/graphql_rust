use async_graphql::dataloader::*;
use async_graphql::*;
use sqlx::FromRow;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PostLoader {
    pub pool: sqlx::PgPool,
}

#[derive(FromRow, Clone)]
pub struct PostRow {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
}

#[async_trait::async_trait]
impl Loader<i32> for PostLoader {
    type Value = PostRow;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let mut result = HashMap::new();

        sqlx::query_as::<_, PostRow>("SELECT * FROM posts WHERE id = ANY($1)")
            .bind(keys)
            .fetch_all(&self.pool)
            .await?
            .iter()
            .for_each(|row| {
                result.insert(row.id, row.to_owned());
            });

        Ok(result)
    }
}
