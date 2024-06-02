use rocket::{serde::json::Json, Responder};
use serde::{Deserialize, Serialize};

use super::docker_models::images::Image;

#[derive(Responder)]
pub enum ApiResponse<T> {
    #[response(status = 200, content_type = "json")]
    Ok(Json<T>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<T>),
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[response(status = 500, content_type = "json")]
    ServerError(Json<T>),
}

#[derive(Serialize, Deserialize)]
pub enum ApiImageResponse {
    Images(Vec<Image>),
    Error(String),
}

pub enum ApiContainerResponse {
    Containers(Vec<String>),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResponse {
    pub result: String,
}
