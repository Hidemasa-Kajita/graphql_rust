use super::users::queries::UserQuery;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);
