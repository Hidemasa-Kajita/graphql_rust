use async_graphql::{http::GraphiQLSource, Schema};
use actix_web::{HttpResponse, http::header::HeaderMap, web, HttpRequest};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use serde::Deserialize;
use super::graphql::schema::SampleSchema;
use super::header::Headers;

fn get_token_from_headers(headers: &HeaderMap) -> Option<Headers> {
    headers
        .get("token")
        .and_then(|value| value.to_str().map(|s| Headers {
            token: s.to_string()
        }).ok())
}

async fn on_connection_init(value: serde_json::Value) -> async_graphql::Result<async_graphql::Data> {
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

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("/")
                .subscription_endpoint("/ws")
                .finish(),
        )
}

pub async fn index(
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

pub async fn index_ws(
    schema: web::Data<SampleSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema))
        .on_connection_init(on_connection_init)
        .start(&req, payload)
}
