use crate::models::api_bodies::ApiResponse;
use crate::services::docker_service;
use crate::Config;
use rocket::get;
use rocket::serde::json::Json;

// ----- IMAGES -----

#[get("/")]
pub async fn get_images(state: &rocket::State<Config>) -> ApiResponse<Vec<String>> {
    let images = docker_service::get_images_linux(&state.docker_socket_path)
        .await
        .unwrap();
    // parse this string into a vector of strings
    let images: Vec<String> = images.split('\n').map(|s| s.to_string()).collect();

    ApiResponse::Ok(Json(images))
}

// ----- CONTAINERS -----

#[get("/")]
pub async fn get_containers(state: &rocket::State<Config>) -> ApiResponse<Vec<String>> {
    let containers = docker_service::get_containers_linux(&state.docker_socket_path)
        .await
        .unwrap();
    // parse this string into a vector of strings
    let containers: Vec<String> = containers.split('\n').map(|s| s.to_string()).collect();

    ApiResponse::Ok(Json(containers))
}
