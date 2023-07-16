use crate::{api::graphql::objects::posts::Post, database::connect::DBPool};

use async_graphql::*;

#[derive(Default)]
pub struct GetPostByIdQuery;

#[Object]
impl GetPostByIdQuery {
    async fn get_post_by_id<'a>(
        &self,
        ctx: &'a Context<'_>,
        id: i32,
    ) -> Result<Option<Post>, async_graphql::Error> {
        let pool = ctx.data_unchecked::<DBPool>();

        Ok(
            match sqlx::query_as::<_, (i32, i32)>("select id, user_id from posts where id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?
            {
                Some(row) => Some(Post {
                    id: row.0,
                    user_id: row.1,
                }),
                None => None,
            },
        )
    }
}
