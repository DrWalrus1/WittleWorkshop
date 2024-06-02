use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod docker_service;

pub async fn connect_db(
    address: &str,
    port: &str,
    username: &str,
    password: &str,
    db_name: &str,
) -> Pool<Postgres> {
    let url = format!("postgres://{username}:{password}@{address}:{port}/{db_name}");
    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect to Postgres");
}
