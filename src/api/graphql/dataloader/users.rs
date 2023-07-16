use async_graphql::dataloader::*;
use async_graphql::*;
use sqlx::FromRow;
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserLoader {
    pub pool: sqlx::PgPool,
}

#[derive(FromRow, Clone)]
pub struct UserRow {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[async_trait::async_trait]
impl Loader<i32> for UserLoader {
    type Value = UserRow;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        let mut result = HashMap::new();

        sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = ANY($1)")
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
