use std::time::Duration;

use sqlx::ConnectOptions;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Error, Pool, Postgres,
};

pub type DBPool = Pool<Postgres>;

pub async fn connect() -> Result<DBPool, Error> {
    let connect_options = PgConnectOptions::new()
        .host("localhost")
        .port(15432)
        .username("postgres")
        .password("postgres")
        .database("postgres");

    // プールの作成と接続
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(
            connect_options
                .log_statements(log::LevelFilter::Debug)
                .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(1)),
        )
        .await?;

    Ok(pool)
}
