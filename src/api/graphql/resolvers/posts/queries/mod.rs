use async_graphql::MergedObject;

use self::{get_post_by_id::GetPostByIdQuery, get_posts::GetPostsQuery};

mod get_post_by_id;
mod get_posts;

#[derive(MergedObject, Default)]
pub struct PostQuery(GetPostByIdQuery, GetPostsQuery);
