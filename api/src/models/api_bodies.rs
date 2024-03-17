use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use crate::commands::docker_commands::DockerImagesCommand;

use super::docker_models::images::Image;

#[derive(Responder)]
pub enum ApiResponse<T> {
    #[response(status = 200, content_type = "json")]
    Ok(Json<T>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<T>)
}

#[derive(Serialize, Deserialize)]
pub struct ContainerRequest {
    pub command: DockerImagesCommand,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DockerImagesResponse {
    pub images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResponse {
    pub result: String,
}