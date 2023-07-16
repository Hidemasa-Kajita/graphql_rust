use crate::database::connect::DBPool;

use async_graphql::*;

use crate::api::graphql::users::object::User;

#[derive(Default)]
struct GetUserByIdQuery;

#[derive(Default)]
pub struct GetUsersQuery;

#[Object]
impl GetUsersQuery {
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
