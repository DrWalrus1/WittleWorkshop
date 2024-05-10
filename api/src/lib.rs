use sqlx::{Pool, Postgres};

pub mod models;
pub mod routes;
pub mod services;

pub struct Config {
    pub docker_socket_path: String,
    pub db_pool: Pool<Postgres>,
}
