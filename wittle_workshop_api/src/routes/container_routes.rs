use rocket::serde::json::Json;
use crate::models::api_bodies::{ContainerRequest, ContainerResponse};

#[post("/container", data = "<container_request>")]
pub fn container(container_request: Json<ContainerRequest>) -> Json<ContainerResponse> {
    let result = ContainerResponse {
        result: container_request.command.to_string(),
    };
    return Json::from(result);
}