use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use super::docker_models::images::Image;

#[derive(Responder)]
pub enum ApiResponse<T> {
    #[response(status = 200, content_type = "json")]
    Ok(Json<T>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<T>),
    #[response(status = 500, content_type = "json")]
    ServerError(Json<T>)
}

#[derive(Serialize, Deserialize)]
pub enum ApiImageResponse {
    Images(Vec<Image>),
    Error(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceResponse {
    pub result: String,
}