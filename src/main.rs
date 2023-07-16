// see: https://github.com/async-graphql/examples/tree/master/actix-web

mod api;
mod database;

use crate::api::graphql::schema::generate_schema;
use crate::api::handler::{graphiql, index, index_ws};
use actix_web::{
    guard,
    web::{self},
    App, HttpServer,
};
use tracing_subscriber::FmtSubscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG) // todo: 環境変数で渡す.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logger");

    let schema = generate_schema().await.expect("Failed genarate schema.");

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
