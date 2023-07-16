use async_graphql::{Object, Context, ComplexObject, SimpleObject};

use super::header::Headers;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: i32
}

#[ComplexObject]
impl User {
    async fn name(&self, _ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        Ok(self.id.to_string() + "name")
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        let pool = ctx.data_unchecked::<crate::database::connect::DBPool>();
        println!("{:?}", pool);
        ctx.data_opt::<Headers>().map(|header| header.token.as_str())
    }
    // async fn user(&self, id: usize) -> User {
    //     let users = vec![
    //         User { id: 1 },
    //         User { id: 2 },
    //         User { id: 3 },
    //         User { id: 4 },
    //         User { id: 5 },
    //     ];

    //     &users[id]
    // }
    async fn users(&self) -> Vec<User> {
        vec![
            User { id: 1 },
            User { id: 2 },
            User { id: 3 },
            User { id: 4 },
            User { id: 5 },
        ]
    }   
}