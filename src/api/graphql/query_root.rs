use crate::database::connect::DBPool;

use super::header::Headers;
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

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: i32,
}

#[ComplexObject]
impl User {
    async fn name(&self, ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        Ok(loader
            .load_one(self.id)
            .await?
            .expect("Not found user.name")
            .name)
    }
    async fn created_at(&self, ctx: &Context<'_>) -> Result<String> {
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        match loader.load_one(self.id).await? {
            Some(user) => Ok(user.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Err("".into()),
        }
    }
    async fn updated_at(&self, ctx: &Context<'_>) -> Result<String> {
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        match loader.load_one(self.id).await? {
            Some(user) => Ok(user.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Err("".into()),
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Headers>()
            .map(|header| header.token.as_str())
    }
    async fn user<'a>(
        &self,
        ctx: &'a Context<'_>,
        id: i32,
    ) -> Result<Option<User>, async_graphql::Error> {
        let pool = ctx.data_unchecked::<DBPool>();

        Ok(
            match sqlx::query_as::<_, (i32,)>("select id from users where id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?
            {
                Some(row) => Some(User { id: row.0 }),
                None => None,
            },
        )
    }
    async fn users<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<User>, async_graphql::Error> {
        let pool = ctx.data_unchecked::<DBPool>();

        Ok(sqlx::query_as::<_, (i32,)>("select id from users")
            .fetch_all(pool)
            .await?
            .iter()
            .map(|&row| User { id: row.0 })
            .collect::<Vec<User>>())
    }
}
