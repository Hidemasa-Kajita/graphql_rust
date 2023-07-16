// see: https://github.com/async-graphql/examples/tree/master/actix-web

use actix_web::{
    guard, http::header::HeaderMap, web::{self}, App, HttpRequest, HttpResponse, HttpServer
};
use async_graphql::{http::GraphiQLSource, SimpleObject, Context, EmptyMutation, Object, Schema, Subscription, futures_util::{self, Stream}, ComplexObject, MergedObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use serde::Deserialize;

pub struct Headers {
    token: String,
}

pub type SampleSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish(),
        )
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Headers> {
    headers
        .get("token")
        .and_then(|value| value.to_str().map(|s| Headers {
            token: s.to_string()
        }).ok())
}

pub async fn on_connection_init(value: serde_json::Value) -> async_graphql::Result<async_graphql::Data> {
    #[derive(Deserialize, Debug)]
    struct Payload {
        token: String,
    }

    // Coerce the connection params into our `Payload` struct so we can
    // validate the token exists in the headers.
    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = async_graphql::Data::default();
        data.insert(Headers { token: payload.token });
        Ok(data)
    } else {
        Err("token is required".into())
    }
}

async fn index(
    schema: web::Data<SampleSchema>,
    req: HttpRequest,
    gql_request: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_request.into_inner();
    if let Some(token) = get_token_from_headers(req.headers()) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

async fn index_ws(
    schema: web::Data<SampleSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema))
        .on_connection_init(on_connection_init)
        .start(&req, payload)
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: i32
}

#[ComplexObject]
impl User {
    async fn name(&self, ctx: &Context<'_>) -> Result<String, async_graphql::Error> {
        Ok(self.id.to_string() + "name")
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
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

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
        if ctx.data::<Headers>()?.token != "123456" {
            return Err("Forbidden".into());
        }
        Ok(futures_util::stream::once(async move { 10 }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(QueryRoot, EmptyMutation, SubscriptionRoot);

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Get()).to(graphiql))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/ws").to(index_ws))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    mod unit {
        #[test]
        fn unit_test() {
            assert_eq!(1, 1)
        }
    }

    mod integration {
        #[test]
        fn integration_test() {
            assert_eq!(1, 1)
        }
    }
}
