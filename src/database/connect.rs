use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};

pub type DBPool = Pool<Postgres>;

pub async fn connect() -> Result<DBPool, Error> {
    Ok(PgPoolOptions::new()
        .max_connections(20)
        .connect("postgres://postgres:postgres@localhost:15432/postgres")
        .await?
    )
}
