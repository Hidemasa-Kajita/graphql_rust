use async_graphql::dataloader::*;
use async_graphql::*;
use sqlx::FromRow;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PostsByUserIdLoader {
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
impl Loader<i32> for PostsByUserIdLoader {
    type Value = Vec<PostRow>;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let mut result = HashMap::new();

        let posts = sqlx::query_as::<_, PostRow>("SELECT * FROM posts WHERE user_id = ANY($1)")
            .bind(keys)
            .fetch_all(&self.pool)
            .await?;

        for post in &posts {
            result
                .entry(post.user_id)
                .or_insert(Vec::new())
                .push(post.to_owned());
        }

        Ok(result)
    }
}
