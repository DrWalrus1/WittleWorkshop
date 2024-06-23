use std::sync::{Arc, Mutex};

use dockworker::Docker;
use tera::Tera;

pub mod models;
pub mod routes;

pub struct Config {
    pub templates: Arc<Mutex<Tera>>,
    pub docker: Docker,
}
