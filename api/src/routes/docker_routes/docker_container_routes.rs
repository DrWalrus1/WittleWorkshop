use crate::{
    models::{
        api_bodies::{ApiResponse, ApiImageResponse},
        docker_models::images::Image,
    },
    services::docker_service,
};
use rocket::serde::json::Json;

#[post("/get")]
pub async fn get_images() -> ApiResponse<ApiImageResponse> {
    let response = match docker_service::get_images_windows().await {
        Ok(response) => response,
        Err(_) => {
            return ApiResponse::ServerError(Json(ApiImageResponse::Error(String::from("Something went wrong getting images from docker"))))
        }
    };
    let response_text = match response.text().await {
        Ok(text) => text,
        Err(_) => {
            return ApiResponse::ServerError(Json(ApiImageResponse::Error(String::from("Something went wrong getting images from docker"))))            
        }
    };

    let parsed_result: Vec<Image> = match serde_json::from_str(&response_text) {
        Ok(parsed_result) => parsed_result,
        Err(_) => {
            return ApiResponse::ServerError(Json(ApiImageResponse::Error(String::from("Something went wrong parsing text from docker"))))            
        }
    };
    ApiResponse::Ok(Json(ApiImageResponse::Images(parsed_result)))
}
