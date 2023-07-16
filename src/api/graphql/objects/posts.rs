use crate::api::graphql::dataloader::posts::PostLoader;
use crate::api::graphql::dataloader::users::UserLoader;
use async_graphql::dataloader::*;
use async_graphql::*;

use super::users::User;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
}

#[ComplexObject]
impl Post {
    async fn title(&self, ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<PostLoader>>();
        Ok(loader
            .load_one(self.id)
            .await?
            .expect("Not found user.name")
            .title)
    }
    async fn content(&self, ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<PostLoader>>();
        Ok(loader
            .load_one(self.id)
            .await?
            .expect("Not found user.name")
            .content)
    }
    async fn user(&self, ctx: &Context<'_>) -> Result<User, async_graphql::Error> {
        let loader = ctx.data_unchecked::<DataLoader<UserLoader>>();
        let user_row = loader
            .load_one(self.user_id)
            .await?
            .expect("Not found user.name");
        Ok(User { id: user_row.id })
    }
}
