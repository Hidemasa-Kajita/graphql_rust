use crate::{api::graphql::objects::posts::Post, database::connect::DBPool};

use async_graphql::*;

#[derive(Default)]
pub struct GetPostsQuery;

#[Object]
impl GetPostsQuery {
    async fn get_posts<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<Post>, async_graphql::Error> {
        let pool = ctx.data_unchecked::<DBPool>();

        Ok(
            sqlx::query_as::<_, (i32, i32)>("select id, user_id from posts")
                .fetch_all(pool)
                .await?
                .iter()
                .map(|&row| Post {
                    id: row.0,
                    user_id: row.1,
                })
                .collect::<Vec<Post>>(),
        )
    }
}
