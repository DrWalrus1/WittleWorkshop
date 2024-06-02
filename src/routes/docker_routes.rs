use crate::models::api_bodies::ApiResponse;
use crate::services::docker_service;
use crate::Config;
use rocket::{get, post};
use rocket::serde::json::Json;

#[get("/<path>")]
pub async fn docker_base_get(path: &str, state: &rocket::State<Config>) -> ApiResponse<Vec<String>> {
    match path {
        "images" => get_images(state).await,
        "containers" => get_containers(state).await,
        _ => ApiResponse::NotFound("Invalid path".to_string()),
    }
}

#[post("/<path>")]
pub async fn docker_base_post(path: &str, state: &rocket::State<Config>) -> ApiResponse<Vec<String>> {
    match path {
        "images" => get_images(state).await,
        "containers" => get_containers(state).await,
        _ => ApiResponse::NotFound("Invalid path".to_string()),
    }
}

// ----- IMAGES -----
pub async fn get_images(state: &rocket::State<Config>) -> ApiResponse<Vec<String>> {
    let images = docker_service::get_images_linux(&state.docker_socket_path)
        .await
        .unwrap();
    // parse this string into a vector of strings
    let images: Vec<String> = images.split('\n').map(|s| s.to_string()).collect();

    ApiResponse::Ok(Json(images))
}

// ----- CONTAINERS -----
pub async fn get_containers(state: &rocket::State<Config>) -> ApiResponse<Vec<String>> {
    let containers = docker_service::get_containers_linux(&state.docker_socket_path)
        .await
        .unwrap();
    // parse this string into a vector of strings
    let containers: Vec<String> = containers.split('\n').map(|s| s.to_string()).collect();

    ApiResponse::Ok(Json(containers))
}
