use async_graphql::MergedObject;

use self::{get_user_by_id::GetUserByIdQuery, get_users::GetUsersQuery};

mod get_user_by_id;
mod get_users;

#[derive(MergedObject, Default)]
pub struct UserQuery(GetUserByIdQuery, GetUsersQuery);
