use super::graphql::resolvers::users::queries::UserQuery;
use async_graphql::MergedObject;

mod dataloader;
mod header;
mod objects;
mod resolvers;
pub mod schema;
mod subscription_root;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);
