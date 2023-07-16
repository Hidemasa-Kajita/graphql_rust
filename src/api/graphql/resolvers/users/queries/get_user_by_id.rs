use crate::database::connect::DBPool;

// use crate::api::graphql::header::Headers;
use async_graphql::*;

use crate::api::graphql::objects::users::User;

#[derive(Default)]
pub struct GetUserByIdQuery;

#[Object]
impl GetUserByIdQuery {
    // async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
    //     ctx.data_opt::<Headers>()
    //         .map(|header| header.token.as_str())
    // }
    async fn get_user_by_id<'a>(
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
}
