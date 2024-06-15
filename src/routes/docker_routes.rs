use crate::models::api_bodies::ApiResponse;
use crate::Config;
use dockworker::container::ContainerFilters;
use dockworker::image::SummaryImage;
use dockworker::Docker;
use rocket::{get, post};
use rocket::serde::json::Json;
use serde::Serialize;
use std::backtrace::Backtrace;

#[get("/<path>")]
pub async fn docker_base_get(path: &str, state: &rocket::State<Config>) -> ApiResponse<Vec<serde_json::Value>> {
    match path {
        "images" => ApiResponse::Ok(Json(convert_to_json(get_images(state).await))),
        "containers" => ApiResponse::Ok(Json(convert_to_json(get_containers(state).await))),
        "networks" => ApiResponse::Ok(Json(convert_to_json(get_networks(state).await))),
        _ => ApiResponse::NotFound("Invalid path".to_string()),
    }
}

fn convert_to_json<T: Serialize>(vec: Vec<T>) -> Vec<serde_json::Value> {
    vec.into_iter().map(|i| serde_json::to_value(i).unwrap()).collect()
}

#[post("/<path>")]
pub async fn docker_base_post(path: &str, state: &rocket::State<Config>) -> ApiResponse<Vec<serde_json::Value>> {
    match path {
        "images" => ApiResponse::Ok(Json(convert_to_json(get_images(state).await))),
        "containers" => ApiResponse::Ok(Json(convert_to_json(get_networks(state).await))),
        _ => ApiResponse::NotFound("Invalid path".to_string()),
    }
}

// ----- IMAGES -----
pub async fn get_images(state: &rocket::State<Config>) -> Vec<SummaryImage> {
    let docker: &Docker = &state.docker;
    let images = docker.images(true).await.unwrap();

    images
}

// ----- CONTAINERS -----
pub async fn get_containers(state: &rocket::State<Config>) -> Vec<dockworker::container::Container> {
    let docker = &state.docker;
    println!("StackTrace: {:?}", Backtrace::force_capture());
    let containers = docker.list_containers(None, None, None, ContainerFilters::default()).await.unwrap();

    containers
}
// ----- Networks -----
pub async fn get_networks(state: &rocket::State<Config>) -> Vec<dockworker::network::Network> {
    let docker = &state.docker;
    let networks = docker.list_networks(Default::default()).await.unwrap();

    networks
}
