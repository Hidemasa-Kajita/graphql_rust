use crate::api::graphql::dataloader::posts_by_user_id::PostsByUserIdLoader;
use crate::api::graphql::dataloader::users::UserLoader;
use async_graphql::dataloader::*;
use async_graphql::*;

use super::posts::Post;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
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
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let loader = ctx.data_unchecked::<DataLoader<PostsByUserIdLoader>>();
        match loader.load_one(self.id).await? {
            Some(posts) => Ok(posts
                .iter()
                .map(|post| Post {
                    id: post.id,
                    user_id: post.user_id,
                })
                .collect::<Vec<Post>>()),
            None => Err("".into()),
        }
    }
}
