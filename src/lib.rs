use sqlx::{Pool, Postgres};
use tera::Tera;

pub mod models;
pub mod routes;
pub mod services;

pub struct Config {
    pub docker_socket_path: String,
    pub templates: Tera
    // pub db_pool: Pool<Postgres>,
}
