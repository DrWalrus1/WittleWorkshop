use dockworker::Docker;
use tera::Tera;

pub mod models;
pub mod routes;


pub struct Config {
    pub templates: Tera,
    pub docker: Docker
}
